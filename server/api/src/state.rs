use axum::extract::FromRef;
use reqwest::Client;
use service::sea_orm::DatabaseConnection;
use tower_cookies::cookie::Key;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub ctx: Client,
    pub db: DatabaseConnection,
    pub key: Key,
}
