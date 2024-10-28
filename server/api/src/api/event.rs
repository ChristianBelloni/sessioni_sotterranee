use axum::{
    extract::{Path, State},
    Json,
};

use aide::{
    axum::{routing::*, ApiRouter as Router},
    transform::TransformOperation,
};

use service::{sea_orm::DbErr, Query};

use crate::{error::Error, extractors::JwtClaims, models::WeeklyEvent, state::AppState};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/:id", get_with(get_event, get_event_docs))
        .with_state(state)
}

async fn get_event(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    jwt: JwtClaims,
) -> Result<Json<WeeklyEvent>, Error> {
    Ok(Query::weekly_event_by_id(&state.db, id)
        .await
        .and_then(|a| a.ok_or(DbErr::RecordNotFound("event not found".to_string())))
        .map(WeeklyEvent::from)
        .map(Json)?)
}

fn get_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get an event by id")
        .response::<201, Json<WeeklyEvent>>()
}
