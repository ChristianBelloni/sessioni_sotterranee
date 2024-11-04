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

pub async fn get_messages(
    State(state): State<AppState>,
    Query(PaginationQuery { limit, offset }): Query<PaginationQuery>,
) {
}
