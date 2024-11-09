use axum::extract::ws::{self, WebSocket};
use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;
use service::sea_orm::DatabaseConnection;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use super::SendMainChatMessage;
use super::WebSocketMessage;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct WSState {
    pub sender: tokio::sync::mpsc::Sender<(i32, ws::Message)>,
    pub clients: Arc<tokio::sync::Mutex<HashMap<i32, SplitSink<WebSocket, ws::Message>>>>,
    pub db: DatabaseConnection,
    persistence_sender: Sender<SendMainChatMessage>,
}

pub struct ClientHandle<'a> {
    user_id: i32,
    sender: Sender<(i32, ws::Message)>,
    state: &'a WSState,
}

impl ClientHandle<'_> {
    pub async fn send(
        &self,
        value: ws::Message,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<(i32, ws::Message)>> {
        self.sender.send((self.user_id, value)).await
    }

    pub async fn publish_others(&self, value: ws::Message) {
        self.state.publish_others(self.user_id, value).await
    }
}

impl WSState {
    pub fn new(
        db: DatabaseConnection,
    ) -> (
        Self,
        Receiver<(i32, ws::Message)>,
        Receiver<SendMainChatMessage>,
    ) {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        let (persistence_sender, persistence_rx) = tokio::sync::mpsc::channel(1024);
        (
            Self {
                sender: tx,
                persistence_sender,
                clients: Default::default(),
                db,
            },
            rx,
            persistence_rx,
        )
    }

    pub async fn subscribe_client(
        &self,
        user_id: i32,
        socket: SplitSink<WebSocket, ws::Message>,
    ) -> ClientHandle {
        self.clients.lock().await.insert(user_id, socket);
        ClientHandle {
            user_id,
            sender: self.sender.clone(),
            state: self,
        }
    }

    pub async fn publish_others(&self, sender: i32, value: ws::Message) {
        let mut lock = self.clients.lock().await;
        let others = lock.iter_mut().filter(|(&id, _)| id != sender);

        futures::stream::iter(others)
            .for_each_concurrent(None, move |(id, client)| {
                let value = value.clone();
                tracing::info!("sending msg to {id} {value:?}");
                async move {
                    _ = client
                        .send(value.clone())
                        .await
                        .inspect_err(|e| tracing::error!(%e));
                }
            })
            .await;
    }

    pub async fn remove_client(&self, id: i32) {
        let sender = self.clients.lock().await.remove(&id);
        if let Some(mut sender) = sender {
            _ = sender.send(ws::Message::Close(None)).await;
        }
    }

    pub async fn run(
        self,
        mut rx: Receiver<(i32, ws::Message)>,
        mut persistence_rx: Receiver<SendMainChatMessage>,
    ) -> Result<(), tokio::task::JoinError> {
        let send_to_clients = tokio::spawn(async move {
            while let Some((user_id, next)) = rx.recv().await {
                if let Some(handle) = self.clients.lock().await.get_mut(&user_id) {
                    _ = handle.send(next).await;
                }
            }
        });

        let connection = self.db.clone();

        let persist_to_db = tokio::spawn(async move {
            while let Some(next_message) = persistence_rx.recv().await {
                _ = service::Mutation::insert_message(
                    &connection,
                    next_message.sender_id,
                    next_message.message_text,
                    next_message.date,
                )
                .await
                .inspect_err(|e| tracing::error!(%e));
            }
        });

        let (send_to_clients, persist_to_db) = tokio::join!(send_to_clients, persist_to_db);
        send_to_clients?;
        persist_to_db?;
        Ok(())
    }
}
