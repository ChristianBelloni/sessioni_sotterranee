use chrono::Duration;
use core::panic;
use reqwest::Client;
use serde::Deserialize;
use serde::Serialize;
use serde_json::json;
use std::future::Future;
use std::sync::Arc;
use std::sync::Mutex;

#[derive(Clone)]
pub struct LogtoAuthenticatedClient {
    logto_endpoint: String,
    logto_tenant_id: String,
    logto_application_id: String,
    logto_application_secret: String,
    token: Arc<Mutex<Option<String>>>,
    client: Client,
}

impl LogtoAuthenticatedClient {
    pub fn new(
        logto_endpoint: String,
        logto_tenant_id: String,
        logto_application_id: String,
        logto_application_secret: String,
    ) -> (Self, impl Future + Send + Sync + 'static) {
        let token = Arc::new(Mutex::new(None));
        let this = Self {
            logto_endpoint,
            logto_tenant_id,
            logto_application_id,
            logto_application_secret,
            token: token.clone(),
            client: Default::default(),
        };
        (this.clone(), tokio::spawn(Self::run_refresher(this)))
    }

    async fn run_refresher(self) {
        loop {
            match self.clone().refresh_token().await {
                Ok(expires_in) => {
                    tokio::time::sleep(expires_in.to_std().unwrap()).await;
                }
                Err(err) => {
                    tracing::error!(%err);
                    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    async fn refresh_token(self) -> Result<Duration, reqwest::Error> {
        let token: Token = self
            .client
            .post(format!("{}/oidc/token", self.logto_endpoint))
            .basic_auth(
                self.logto_application_id,
                Some(self.logto_application_secret),
            )
            .form(&[
                ("grant_type", "client_credentials"),
                (
                    "resource",
                    &format!("https://{}.logto.app/api", self.logto_tenant_id),
                ),
                ("scope", "all"),
            ])
            .send()
            .await?
            .json()
            .await?;

        self.token.lock().unwrap().replace(token.access_token);

        Ok(Duration::seconds(token.expires_in - 5))
    }

    async fn patch<T: Serialize, R: for<'a> Deserialize<'a>>(
        &self,
        url: String,
        request: T,
    ) -> Result<R, reqwest::Error> {
        let token: Option<String> = self.token.lock().unwrap().clone();
        let Some(token) = token else {
            panic!("token not found");
        };

        let result = self
            .client
            .patch(format!("{}{}", self.logto_endpoint, url))
            .bearer_auth(token)
            .json(&request)
            .send()
            .await?
            .json()
            .await?;
        Ok(result)
    }

    pub async fn update_user_username(&self, logto_user_id: String, username: String) {
        let _: Result<(), _> = self
            .patch(
                format!("/api/users/{logto_user_id}"),
                json! {{
                    "username": username
                }},
            )
            .await;
    }
}

#[derive(Serialize, Deserialize)]
struct Token {
    pub access_token: String,
    pub expires_in: i64,
    pub token_type: String,
    pub scope: String,
}
