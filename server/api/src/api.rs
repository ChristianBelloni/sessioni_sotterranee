use aide::axum::ApiRouter as Router;
use axum::extract::State;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::{extractors::LogtoWebHook, state::AppState};

mod event;
mod main_chat;
mod street_jam;
mod user;

#[derive(JsonSchema, Serialize, Deserialize)]
pub struct PaginationQuery {
    pub limit: u64,
    pub offset: u64,
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .nest_api_service("/events", event::router(state.clone()))
        .nest_api_service("/users", user::router(state.clone()))
        .nest_api_service("/messages", main_chat::router(state.clone()))
        .route("/logto_wh", axum::routing::post(logto_wh))
        .with_state(state)
}

async fn logto_wh(State(state): State<AppState>, LogtoWebHook(body): LogtoWebHook) {
    match body {
        crate::extractors::WHUserEvents::UserCreated(whuser_event) => {
            if let Some(user_id) = whuser_event.data.map(|a| a.id) {
                _ = service::Mutation::insert_user(&state.db, user_id, Default::default())
                    .await
                    .inspect_err(|e| tracing::error!(%e));
            }
        }
        crate::extractors::WHUserEvents::UserDeleted(whuser_event) => {
            if let Some(user_id) = whuser_event.data.map(|a| a.id) {
                _ = service::Mutation::delete_user_by_logto_id(&state.db, user_id)
                    .await
                    .inspect_err(|e| tracing::error!(%e));
            }
        }
        crate::extractors::WHUserEvents::UserUpdated(data) => {
            tracing::info!(?data, "update user");
            if let Some((user_id, username)) = data.data.and_then(|a| Some((a.id, a.username?))) {
                let (user, _) = service::Query::get_user_from_logto_id(&state.db, user_id)
                    .await
                    .unwrap()
                    .unwrap();
                tracing::info!("got user to update");
                _ = service::Mutation::update_username(&state.db, user.id, username).await;
            }
        }
    }
}
