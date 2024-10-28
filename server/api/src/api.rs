use aide::axum::ApiRouter as Router;

use crate::state::AppState;

mod auth;
mod event;
mod user;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest_api_service("/user", user::router(state.clone()))
        .nest_api_service("/auth", auth::router(state.clone()))
        .nest_api_service("/event", event::router(state.clone()))
        .with_state(state)
}
