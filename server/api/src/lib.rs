use aide::{axum::ApiRouter as Router, openapi::OpenApi};
use axum::{error_handling::HandleErrorLayer, http::Uri, response::IntoResponse, Extension};
use docs::docs_routes;
use migration::{Migrator, MigratorTrait};
use oauth::build_oauth_client;
use service::sea_orm::Database;
use state::{AppState, Configuration};
use std::{env, sync::Arc};
use tower::ServiceBuilder;
use tower_cookies::{cookie::Key, CookieManagerLayer};
use tower_sessions::{
    cookie::{time::Duration, SameSite},
    Expiry, MemoryStore, SessionManagerLayer,
};

use axum_oidc::{
    error::MiddlewareError, EmptyAdditionalClaims, OidcAuthLayer, OidcClaims, OidcLoginLayer,
    OidcRpInitiatedLogout,
};

mod api;
mod docs;
mod error;
pub(crate) mod extractors;
pub mod models;
mod oauth;
mod state;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let oidc_id = env::var("LOGTO_OIDC_ID").expect("LOGTO_OIDC_ID is not set in .env file");
    let oidc_secret =
        env::var("LOGTO_OIDC_SECRET").expect("LOGTO_OIDC_SECRET is not set in .env file");
    let google_client_id =
        env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID is not set in .env file");
    let google_client_secret =
        env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET is not set in .env file");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let mut o_api = OpenApi::default();

    let configuration = Configuration {
        redirect_url: format!("http://{host}:{port}/api/auth/google_callback"),
        host,
        port: port.parse().unwrap(),
        oauth_id: "".to_string(),
    };

    let oauth_client = build_oauth_client(
        configuration.clone(),
        google_client_id,
        google_client_secret,
    );

    let state = AppState {
        ctx: ::reqwest::Client::default(),
        db: conn,
        configuration,
        key: Key::generate(),
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(false)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

    let oidc_auth_client = OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::from_maybe_shared("http://127.0.0.1:8080").expect("valid APP_URL"),
        "http://localhost:3002/oidc".to_string(),
        oidc_id,
        Some(oidc_secret),
        vec![],
    )
    .await
    .unwrap();

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            e.into_response()
        }))
        .layer(oidc_auth_client);

    let app = Router::new()
        .nest_api_service("/api", api::router(state.clone()))
        .nest_api_service("/docs", docs_routes(state.clone()))
        .finish_api(&mut o_api)
        .layer(CookieManagerLayer::new())
        .layer(Extension(Arc::new(o_api)))
        .layer(Extension(oauth_client))
        .layer(oidc_auth_service)
        .layer(session_layer)
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}
