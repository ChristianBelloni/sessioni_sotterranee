use axum::extract::FromRef;
use reqwest::Client;
use service::sea_orm::DatabaseConnection;

use crate::{logto_client::LogtoAuthenticatedClient, ws::WSState};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub ctx: Client,
    pub db: DatabaseConnection,
    pub ws_state: WSState,
    pub logto_client: LogtoAuthenticatedClient,
}
