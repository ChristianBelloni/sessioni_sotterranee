use std::collections::HashMap;

use aide::{
    openapi::{Parameter, ParameterData},
    operation::add_parameters,
    OperationInput,
};
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, FromRequestParts, Request},
    http::request::Parts,
    Extension, RequestExt, RequestPartsExt,
};

use axum_oidc::{EmptyAdditionalClaims, OidcClaims};
use jsonwebtoken::TokenData;
use migration::{DbErr, ExprTrait};
use oidc_jwt_validator::{ValidationSettings, Validator};
use reqwest::Client;

use crate::{models::User, state::AppState};
use service::Query as DbQuery;

mod logto_wh;

pub use logto_wh::*;

#[derive(Clone)]
pub struct JwtClaims(pub OidcClaims<EmptyAdditionalClaims>);

impl OperationInput for JwtClaims {
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
        let s = ctx.schema.subschema_for::<String>();
        add_parameters(
            ctx,
            operation,
            [Parameter::Header {
                parameter_data: ParameterData {
                    name: "Authorization".to_string(),
                    description: Some("Jwt Bearer token".to_string()),
                    required: true,
                    format: aide::openapi::ParameterSchemaOrContent::Schema(
                        aide::openapi::SchemaObject {
                            json_schema: s,
                            example: None,
                            external_docs: None,
                        },
                    ),
                    extensions: Default::default(),
                    deprecated: None,
                    example: None,
                    examples: Default::default(),
                    explode: None,
                },
                style: aide::openapi::HeaderStyle::Simple,
            }],
        );
    }
}

impl OperationInput for User {
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
        JwtClaims::operation_input(ctx, operation)
    }
}

#[async_trait]
impl<T> FromRequestParts<T> for JwtClaims
where
    OidcClaims<EmptyAdditionalClaims>: FromRequestParts<T>,
    T: Send + Sync + 'static,
{
    type Rejection = <OidcClaims<EmptyAdditionalClaims> as FromRequestParts<T>>::Rejection;
    async fn from_request_parts(parts: &mut Parts, state: &T) -> Result<Self, Self::Rejection> {
        Ok(Self(OidcClaims::from_request_parts(parts, state).await?))
    }
}

#[async_trait]
impl FromRequestParts<AppState> for User {
    type Rejection = crate::error::Error;
    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        tracing::info!("authenticating user");
        let claims = OidcToken::from_request_parts(parts, state).await?.0.claims;

        let user_id = match claims.get("sub").unwrap() {
            serde_json::Value::String(user_id) => user_id.to_string(),
            _ => unreachable!("user id should be a string"),
        };
        tracing::info!("got user claims");
        let user = User::from(
            DbQuery::get_user_from_logto_id(&state.db, user_id)
                .await?
                .ok_or(DbErr::RecordNotFound("user not found".to_string()))?,
        );
        Ok(user)
    }
}

#[derive(Clone)]
pub struct OidcValidationLayer {
    pub settings: Validator,
}
struct OidcToken(TokenData<HashMap<String, serde_json::Value>>);

#[async_trait]
impl FromRequestParts<AppState> for OidcToken {
    type Rejection = crate::error::Error;
    async fn from_request_parts(parts: &mut Parts, _: &AppState) -> Result<Self, Self::Rejection> {
        tracing::info!("authenticating user");
        let validator = parts.extensions.get::<OidcValidationLayer>().unwrap();
        let token = validator
            .settings
            .validate::<HashMap<String, serde_json::Value>>(
                parts
                    .headers
                    .get("Authorization")
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .split(' ')
                    .last()
                    .unwrap(),
            )
            .await
            .unwrap();

        Ok(OidcToken(token))
    }
}
