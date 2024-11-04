use axum::extract::FromRef;
use reqwest::Client;
use service::sea_orm::DatabaseConnection;
use tower_cookies::cookie::Key;

use crate::{logto_access_token::LogtoAuthenticatedClient, ws::WSState};

#[derive(Clone, FromRef)]
pub struct AppState {
    pub ctx: Client,
    pub db: DatabaseConnection,
    pub ws_state: WSState, // pub key: Key,
    pub logto_client: LogtoAuthenticatedClient,
}
