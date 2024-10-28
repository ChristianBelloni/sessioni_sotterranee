use aide::OperationOutput;
use axum::{http::StatusCode, response::IntoResponse};
use migration::DbErr;

use oauth2::{basic::BasicErrorResponseType, RequestTokenError, StandardErrorResponse};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Database(#[from] DbErr),
    #[error(transparent)]
    Token(
        #[from]
        RequestTokenError<
            oauth2::reqwest::Error<reqwest::Error>,
            StandardErrorResponse<BasicErrorResponseType>,
        >,
    ),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

impl OperationOutput for Error {
    type Inner = Self;
    fn inferred_responses(
        _ctx: &mut aide::gen::GenContext,
        operation: &mut aide::openapi::Operation,
    ) -> Vec<(Option<u16>, aide::openapi::Response)> {
        if let Some(responses) = operation
            .responses
            .as_ref()
            .and_then(|a| a.responses.first())
            .and_then(|a| Some((Some(a.0.to_string().parse().ok()?), a.1.as_item()?.clone())))
        {
            vec![responses.clone()]
        } else {
            vec![(Some(500), Default::default())]
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        match self {
            Error::Database(db) => {
                tracing::error!(%db);
                if let DbErr::RecordNotFound(_) = db {
                    (StatusCode::NOT_FOUND, "Record not found").into_response()
                } else {
                    (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        "Error while processing request",
                    )
                        .into_response()
                }
            }
            Error::Token(err) => {
                tracing::error!(%err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error while processing request",
                )
                    .into_response()
            }
            Error::Reqwest(err) => {
                tracing::error!(%err);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Error while processing request",
                )
                    .into_response()
            }
        }
    }
}
