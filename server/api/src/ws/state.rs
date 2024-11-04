use axum::extract::ws::{self, WebSocket};
use futures::stream::SplitSink;
use futures::SinkExt;
use futures::StreamExt;
use tokio::sync::mpsc::Receiver;
use tokio::sync::mpsc::Sender;

use super::WebSocketMessage;
use std::{collections::HashMap, sync::Arc};

#[derive(Clone)]
pub struct WSState {
    pub sender: tokio::sync::mpsc::Sender<(i32, ws::Message)>,
    pub clients: Arc<tokio::sync::Mutex<HashMap<i32, SplitSink<WebSocket, ws::Message>>>>,
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

    pub async fn publish_others(&self, value: ws::Message) {}
}

impl WSState {
    pub fn new() -> (Self, Receiver<(i32, ws::Message)>) {
        let (tx, rx) = tokio::sync::mpsc::channel(1024);
        (
            Self {
                sender: tx,
                clients: Default::default(),
            },
            rx,
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
            .for_each_concurrent(None, move |(_, client)| {
                let value = value.clone();
                async move {
                    _ = client.send(value.clone()).await;
                }
            })
            .await;
    }

    pub async fn run(
        self,
        mut rx: Receiver<(i32, ws::Message)>,
    ) -> Result<(), tokio::task::JoinError> {
        tokio::spawn(async move {
            while let Some((user_id, next)) = rx.recv().await {
                if let Some(handle) = self.clients.lock().await.get_mut(&user_id) {
                    _ = handle.send(next).await;
                }
            }
        })
        .await
    }
}
