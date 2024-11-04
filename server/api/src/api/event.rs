use super::PaginationQuery;
use crate::{
    error::Error,
    models::{Event, EventsForUser, User},
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
use service::{sea_orm::DbErr, Query as DbQuery};

pub fn router(state: AppState) -> Router {
    Router::new()
        .api_route("/", get_with(next_events, next_events_docs))
        .api_route("/:id", get_with(get_event, get_event_docs))
        .api_route(
            "/forme",
            get_with(get_events_for_user, get_events_for_user_docs),
        )
        .with_state(state)
}

async fn next_events(
    State(state): State<AppState>,
    Query(PaginationQuery { offset, limit }): Query<PaginationQuery>,
) -> Result<Json<Vec<Event>>, Error> {
    let events = DbQuery::get_next_evnts(&state.db, offset, limit)
        .await?
        .into_iter()
        .map(Event::from)
        .collect();
    Ok(Json(events))
}

fn next_events_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get paginated upcoming events")
}

#[derive(Deserialize, JsonSchema)]
struct SelectEvent {
    pub id: i32,
}

async fn get_event(
    State(state): State<AppState>,
    Path(SelectEvent { id }): Path<SelectEvent>,
) -> Result<Json<Event>, Error> {
    Ok(DbQuery::event_by_id(&state.db, id)
        .await
        .and_then(|a| a.ok_or(DbErr::RecordNotFound("event not found".to_string())))
        .map(Event::from)
        .map(Json)?)
}

async fn get_events_for_user(
    State(state): State<AppState>,
    Query(PaginationQuery { offset, limit }): Query<PaginationQuery>,
    user: User,
) -> Result<Json<EventsForUser>, Error> {
    let result = DbQuery::get_evnts_for_user(&state.db, user.id, offset, limit)
        .await?
        .into();
    Ok(Json(result))
}

fn get_events_for_user_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get paginated attending and interested events for the current user")
}

fn get_event_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get an event by id")
        .parameter("id", move |transform: TransformParameter<'_, i32>| {
            transform.description("Event id")
        })
}
