use super::PaginationQuery;
use crate::{
    error::Error,
    models::{Event, EventsForUser, User},
    state::AppState,
};
use aide::{
    axum::{routing::*, ApiRouter as Router},
    transform::TransformOperation,
};
use axum::{
    extract::{Path, Query, State},
    Json,
};
use service::{sea_orm::DbErr, Query as DbQuery};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/", get(get_messages))
        .with_state(state)
}

pub async fn get_messages(
    State(state): State<AppState>,
    Query(PaginationQuery { limit, offset }): Query<PaginationQuery>,
) {
    DbQuery::get_messages(&state.db, offset, limit).await;
}
