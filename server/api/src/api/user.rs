use crate::{
    error::Error,
    models::{CompleteRegistrationRequest, User},
    state::AppState,
};
use aide::{
    axum::{routing::*, ApiRouter as Router},
    transform::{TransformOperation, TransformParameter},
};
use axum::extract::{Json, Query, State};
use service::{sea_orm::DbErr, Query as DbQuery};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/me", get(get_username))
        .api_route("/set_username", patch_with(set_username, set_username_docs))
        .with_state(state)
}

async fn get_username(user: User) -> Result<Json<User>, Error> {
    Ok(Json(user))
}

async fn set_username(
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

fn set_username_docs(op: TransformOperation) -> TransformOperation {
    op.description("Set username")
        .parameter("id", move |transform: TransformParameter<'_, i32>| {
            transform.description("Event id")
        })
}
