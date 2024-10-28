use aide::{
    openapi::{Parameter, ParameterData},
    operation::add_parameters,
    OperationInput,
};
use axum::{async_trait, extract::FromRequestParts, http::request::Parts};

use axum_oidc::{EmptyAdditionalClaims, OidcClaims};

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
