use aide::{
    openapi::{Parameter, ParameterData},
    operation::add_parameters,
    OperationInput,
};
use axum::{
    async_trait,
    body::Bytes,
    extract::{FromRequest, Request},
};
use serde::{Deserialize, Serialize};
use service::sea_orm::prelude::{DateTimeUtc, DateTimeWithTimeZone};

pub struct LogtoWebHook(pub WHUserEvents);

#[derive(Clone)]
pub struct LogtoWebhookSecret(pub String);

#[async_trait]
impl<T> FromRequest<T> for LogtoWebHook {
    type Rejection = crate::error::Error;
    async fn from_request(request: Request, _: &T) -> Result<Self, Self::Rejection> {
        use hmac::{Hmac, Mac};
        use sha2::Sha256;

        type HmacSha256 = Hmac<Sha256>;

        let Some(header) = request.headers().get("logto-signature-sha-256") else {
            return Err(crate::error::Error::UnAuthorized);
        };

        let header = header.clone();
        let header = header.to_str().unwrap();

        tracing::info!(%header);

        let Some(LogtoWebhookSecret(secret)) = request.extensions().get() else {
            return Err(crate::error::Error::UnAuthorized);
        };

        tracing::info!(%secret);

        let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();

        let bytes: Bytes = axum::body::to_bytes(request.into_body(), usize::MAX)
            .await
            .unwrap();

        mac.update(&bytes);
        let mac_bytes = mac.finalize().into_bytes().to_vec();

        let signature = hex::encode(mac_bytes);

        if signature == header {
            tracing::info!("valid");
        } else {
            return Err(crate::error::Error::UnAuthorized);
        }

        let tmp = String::from_utf8_lossy(&bytes);
        tracing::info!(%tmp);

        let data: WHUserEvents = serde_json::from_slice(&bytes).unwrap();

        Ok(LogtoWebHook(data))
    }
}

impl OperationInput for LogtoWebHook {
    fn operation_input(ctx: &mut aide::gen::GenContext, operation: &mut aide::openapi::Operation) {
        let s = ctx.schema.subschema_for::<String>();
        add_parameters(
            ctx,
            operation,
            [Parameter::Header {
                parameter_data: ParameterData {
                    name: "logto-signature-sha-256".to_string(),
                    description: Some("Sha256 signature".to_string()),
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
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "event")]
pub enum WHUserEvents {
    #[serde(rename = "User.Deleted")]
    UserDeleted(WHUserEvent),
    #[serde(rename = "User.Created")]
    UserCreated(WHUserEvent),
    #[serde(rename = "User.Data.Updated")]
    UserUpdated(WHUserEvent),
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WHUserEvent {
    pub hook_id: String,
    pub created_at: Option<DateTimeUtc>,
    pub session_id: Option<String>,
    pub ip: Option<String>,
    pub path: Option<String>,
    pub method: Option<String>,
    pub status: Option<i64>,
    pub data: Option<WHUser>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WHUserEventParams {
    pub id: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WHUser {
    pub id: String,
    pub username: Option<String>,
    pub primary_email: Option<String>,
    pub last_sign_in_at: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}
