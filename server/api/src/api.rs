use aide::axum::ApiRouter as Router;

use crate::state::AppState;

mod event;

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest_api_service("/event", event::router(state.clone()))
        .with_state(state)
}
