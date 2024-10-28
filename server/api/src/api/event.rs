use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
};

use aide::{
    axum::{routing::*, ApiRouter as Router, IntoApiResponse},
    transform::TransformOperation,
};

use service::{sea_orm::DbErr, Query};

use crate::{error::Error, models::WeeklyEvent, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/:id", get_with(get_event, get_event_docs))
        .with_state(state)
}

async fn get_event(State(state): State<AppState>, Path(id): Path<i32>) -> impl IntoApiResponse {
    Query::weekly_event_by_id(&state.db, id)
        .await
        .and_then(|a| a.ok_or(DbErr::RecordNotFound("event not found".to_string())))
        .map_err(Error::from)
        .map(WeeklyEvent::from)
        .map(Json)
        .into_response()
}

fn get_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get an event by id")
        .response::<201, Json<WeeklyEvent>>()
}
