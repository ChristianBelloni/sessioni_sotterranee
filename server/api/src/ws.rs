use crate::state::AppState;
use aide::axum::IntoApiResponse;
use axum::extract::{
    ws::{self, Message, WebSocket},
    State, WebSocketUpgrade,
};
use futures::{SinkExt, StreamExt};
use models::*;

pub mod models;
mod state;

pub use state::WSState;

pub async fn websocket(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoApiResponse {
    tracing::info!("new ws connection");
    ws.on_upgrade(|socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut sink, mut stream) = socket.split();

    let user_id = match stream.next().await {
        Some(Ok(ws::Message::Binary(data))) => match serde_json::from_slice(&data) {
            Ok(WebSocketMessage::IdentifyClient(IdentifyClient { user_id })) => user_id,
            _ => {
                tracing::info!("received other message instead of identify client");
                _ = sink.send(ws::Message::Close(None)).await;
                return;
            }
        },
        Some(Ok(ws::Message::Text(data))) => match serde_json::from_str(&data) {
            Ok(WebSocketMessage::IdentifyClient(IdentifyClient { user_id })) => user_id,
            _ => {
                tracing::info!("received other message instead of identify client");
                _ = sink.send(ws::Message::Close(None)).await;
                return;
            }
        },
        _ => {
            tracing::info!("received other message instead of identify client");
            _ = sink.send(ws::Message::Close(None)).await;
            return;
        }
    };

    let username = match service::Query::get_user(&state.db, user_id).await {
        Ok(Some(user)) => user.0.username,
        _ => {
            tracing::error!("failed to retrieve user from db");
            return;
        }
    };

    let sink = state.ws_state.subscribe_client(user_id, sink).await;

    while let Some(next) = stream.next().await {
        let next = match next {
            Ok(data) => data,
            Err(err) => {
                tracing::error!(%err);
                continue;
            }
        };
        let msg = match next {
            axum::extract::ws::Message::Text(data) => {
                serde_json::from_str(&data).expect("serialization successfull")
            }
            axum::extract::ws::Message::Binary(data) => {
                serde_json::from_slice(&data).expect("serialization successfull")
            }
            axum::extract::ws::Message::Ping(data) => {
                _ = sink.send(ws::Message::Pong(data)).await;
                continue;
            }
            axum::extract::ws::Message::Pong(_) => {
                tracing::error!("received pong from client");
                continue;
            }
            axum::extract::ws::Message::Close(close_frame) => {
                tracing::info!("received close frame from client {:?}", close_frame);
                break;
            }
        };

        match msg {
            WebSocketMessage::SentMainChatMessage(new_message) => {
                tracing::info!("recvd message {:?}", new_message);
                _ = service::Mutation::insert_message(
                    &state.db,
                    new_message.sender_id,
                    new_message.message_text.clone(),
                    new_message.date,
                )
                .await;
                sink.publish_others(Message::Text(
                    serde_json::to_string(&WebSocketMessage::PublishedMainChatMessage(
                        MainChatMessage {
                            sender: MessageSenderUsername::Other(username.clone()),
                            message_text: new_message.message_text,
                            date: new_message.date,
                        },
                    ))
                    .unwrap(),
                ))
                .await;
            }
            WebSocketMessage::PublishedMainChatMessage(_) => continue,
            WebSocketMessage::RequestMainChatHistory(request_main_chat_history) => {
                let requester_id = request_main_chat_history.user_id;
                if requester_id != user_id {
                    tracing::error!("requester id is different from identify_client id");
                    break;
                }

                let messages = service::Query::get_messages(&state.db, 0, 100)
                    .await
                    .unwrap_or_default();

                let messages = messages
                    .into_iter()
                    .map(|msg| MainChatMessage {
                        sender: if msg.user_id == user_id {
                            MessageSenderUsername::Me
                        } else {
                            todo!()
                        },
                        message_text: msg.message_text,
                        date: msg.date,
                    })
                    .collect();

                _ = sink
                    .send(ws::Message::Text(
                        serde_json::to_string(&WebSocketMessage::PublishedMainChatHistory(
                            PublishedMainchatHistory { user_id, messages },
                        ))
                        .unwrap(),
                    ))
                    .await;
            }
            WebSocketMessage::PublishedMainChatHistory(_) => continue,
            WebSocketMessage::IdentifyClient(IdentifyClient {
                user_id: new_user_id,
            }) => {
                tracing::warn!("received duplicate identify client");
                if user_id != new_user_id {
                    tracing::error!("different user id in duplicate identify_client");
                    _ = sink.send(ws::Message::Close(None)).await;
                    break;
                }
            }
        }
    }
    tracing::info!("removing client");

    state.ws_state.remove_client(user_id).await;
}
