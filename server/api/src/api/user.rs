use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Extension, Json,
};

use aide::{
    axum::{routing::*, ApiRouter as Router, IntoApiResponse},
    transform::TransformOperation,
};

use service::{sea_orm::DbErr, Mutation, Query};

use crate::{
    error::Error,
    models::{CreateUser, User},
    state::{AppState, Configuration},
};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/", put_with(put_user, put_user_docs))
        .api_route("/:id", get_with(get_user, get_user_docs))
        .api_route("/:id", delete_with(delete_user, delete_user_docs))
        .with_state(state)
}

async fn put_user(
    State(state): State<AppState>,
    Json(new_user): Json<CreateUser>,
) -> impl IntoApiResponse {
    let CreateUser {
        username,
        email,
        role_kind,
    } = new_user;

    Mutation::insert_user(&state.db, &username, &email, role_kind.into())
        .await
        .map(User::from)
        .map(Json)
        .map_err(Error::from)
        .into_response()
}

fn put_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Create a new user")
        .response::<201, Json<User>>()
}

async fn get_user(State(state): State<AppState>, Path(user_id): Path<i32>) -> impl IntoApiResponse {
    let user = Query::user_by_id(&state.db, user_id)
        .await
        .and_then(|a| a.ok_or(DbErr::RecordNotFound(format!("{}", user_id))))
        .map_err(Error::from)
        .map(User::from)
        .map(Json)
        .into_response();

    user
}

fn get_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a user by its id")
        .response::<201, Json<User>>()
        .response::<500, String>()
}

async fn delete_user(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> impl IntoApiResponse {
    Mutation::delete_user(&state.db, user_id)
        .await
        .map(|_| ())
        .map_err(Error::from)
        .into_response()
}

fn delete_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Delete a user by its id")
        .response::<201, ()>()
}
