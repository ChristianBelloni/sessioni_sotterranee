use aide::{axum::ApiRouter as Router, openapi::OpenApi};
use axum::{error_handling::HandleErrorLayer, http::Uri, response::IntoResponse, Extension};
use axum_oidc::{error::MiddlewareError, EmptyAdditionalClaims, OidcAuthLayer};
use docs::docs_routes;
use extractors::{LogtoWebhookSecret, OidcValidationLayer};
use futures::FutureExt;
use logto_client::LogtoAuthenticatedClient;
use migration::{Migrator, MigratorTrait};
use oidc_jwt_validator::{cache::Strategy, ValidationSettings, Validator};
use service::sea_orm::Database;
use state::AppState;
use std::{env, future::IntoFuture, sync::Arc};
use tower::ServiceBuilder;
use tower_cookies::CookieManagerLayer;
use tower_sessions::{
    cookie::{time::Duration, SameSite},
    Expiry, MemoryStore, SessionManagerLayer,
};
use ws::WSState;

mod api;
mod docs;
mod error;
pub(crate) mod extractors;
mod logto_client;
pub mod models;
mod state;
pub mod ws;

#[tokio::main]
pub async fn start() -> anyhow::Result<()> {
    // env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();

    let logto_tenant_id =
        env::var("LOGTO_TENANT_ID").expect("LOGTO_TENANT_ID is not set in .env file");

    let logto_application_id =
        env::var("LOGTO_APPLICATION_ID").expect("LOGTO_APPLICATION_ID is not set in .env file");

    let logto_application_secret = env::var("LOGTO_APPLICATION_SECRET")
        .expect("LOGTO_APPLICATION_SECRET is not set in .env file");

    let logto_endpoint =
        env::var("LOGTO_ENDPOINT").expect("LOGTO_ENDPOINT is not set in .env file");

    let logto_issuer_url =
        env::var("LOGTO_ISSUER_URL").expect("LOGTO_APP_URL is not set in .env file");

    let oidc_id = env::var("LOGTO_OIDC_ID").expect("LOGTO_OIDC_ID is not set in .env file");
    let oidc_secret =
        env::var("LOGTO_OIDC_SECRET").expect("LOGTO_OIDC_SECRET is not set in .env file");

    let logto_wh_secret =
        env::var("LOGTO_WH_SECRET").expect("LOGTO_WH_SECRET is not set in .env file");

    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{host}:{port}");

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");
    Migrator::up(&conn, None).await.unwrap();

    let mut o_api = OpenApi::default();

    let (ws_state, ws_rx, persistence_rx) = WSState::new(conn.clone());

    let (logto_client, refresher) = LogtoAuthenticatedClient::new(
        logto_endpoint,
        logto_tenant_id,
        logto_application_id,
        logto_application_secret,
    );

    let state = AppState {
        ctx: ::reqwest::Client::default(),
        db: conn,
        ws_state: ws_state.clone(), // key: Key::generate(),
        logto_client,
    };

    let session_store = MemoryStore::default();
    let session_layer = SessionManagerLayer::new(session_store)
        .with_secure(true)
        .with_same_site(SameSite::Lax)
        .with_expiry(Expiry::OnInactivity(Duration::seconds(120)));

    let oidc_auth_client = OidcAuthLayer::<EmptyAdditionalClaims>::discover_client(
        Uri::from_maybe_shared(format!("http://{host}:{port}/api")).expect("valid APP_URL"),
        logto_issuer_url.clone(),
        oidc_id,
        Some(oidc_secret),
        vec!["all".to_string()],
    )
    .await
    .unwrap();

    let oidc_auth_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            tracing::error!(%e);
            e.into_response()
        }))
        .layer(oidc_auth_client);

    let client = reqwest::ClientBuilder::new()
        .timeout(std::time::Duration::from_secs(2))
        .build()
        .unwrap();

    let mut settings = ValidationSettings::new();

    settings.set_issuer(&[logto_issuer_url.clone()]);
    settings.set_audience(&["virc2uruta8tetclpuu03"]);

    let validator = Validator::new(logto_issuer_url, client, Strategy::Automatic, settings)
        .await
        .unwrap();

    let app = Router::new()
        .nest_api_service("/api", api::router(state.clone()))
        .nest_service("/docs", docs_routes(state.clone()))
        .api_route("/ws", aide::axum::routing::get(ws::websocket))
        .finish_api(&mut o_api)
        .layer(CookieManagerLayer::new())
        .layer(Extension(Arc::new(o_api)))
        .layer(oidc_auth_service)
        .layer(session_layer)
        .layer(Extension(LogtoWebhookSecret(logto_wh_secret)))
        .layer(Extension(OidcValidationLayer {
            settings: validator,
        }))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&server_url).await.unwrap();

    tokio::try_join!(
        axum::serve(listener, app)
            .into_future()
            .then(|a| async move { anyhow::Ok(a?) }),
        ws_state
            .run(ws_rx, persistence_rx)
            .then(|a| async move { Ok(a?) }),
        refresher.then(|a| async move { Ok(a) })
    )?;

    Ok(())
}
