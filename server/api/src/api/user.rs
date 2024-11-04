use crate::{
    error::Error,
    models::{CompleteRegistrationRequest, Event, EventsForUser, User},
    state::AppState,
};
use aide::{
    axum::{routing::*, ApiRouter as Router},
    transform::{TransformOperation, TransformParameter},
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use schemars::JsonSchema;
use serde::Deserialize;
use service::{sea_orm::DbErr, Mutation as DbMutation, Query as DbQuery};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route(
            "/set_username",
            patch_with(complete_registration, complete_registration_docs),
        )
        .with_state(state)
}

async fn complete_registration(
    State(state): State<AppState>,
    user: User,
    Query(CompleteRegistrationRequest { username }): Query<CompleteRegistrationRequest>,
) -> Result<(), Error> {
    let (user, _) = DbQuery::get_user(&state.db, user.id)
        .await?
        .ok_or(DbErr::RecordNotFound("user not found".to_string()))?;

    state
        .logto_client
        .update_user_username(user.log_to_id, username)
        .await;
    Ok(())
}

fn complete_registration_docs(op: TransformOperation) -> TransformOperation {
    op.description("complete user registration")
        .parameter("id", move |transform: TransformParameter<'_, i32>| {
            transform.description("Event id")
        })
}
