use axum::extract::FromRef;
use reqwest::Client;
use service::sea_orm::DatabaseConnection;
use tower_cookies::cookie::Key;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub ctx: Client,
    pub db: DatabaseConnection,
    pub configuration: Configuration,
    pub key: Key,
}

#[derive(Clone)]
pub struct Configuration {
    pub host: String,
    pub port: u16,
    pub oauth_id: String,
    pub redirect_url: String,
}
