use async_chat::FromServer;
use async_std::task;
use std::sync::Arc;
use tokio::sync::broadcast::{self, error::RecvError};

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ignored = self.sender.send(message);
    }

    pub fn new(name: Arc<String>) -> Self {
        let (sender, _reciever) = broadcast::channel(1000);
        Group { name, sender }
    }
}

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop {
        let packet = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => {
                FromServer::Error(format!("Dropped {} messages from {}.", n, group_name))
            }
            Err(RecvError::Closed) => break,
        };

        if outbound.send(packet).await.is_err() {
            break;
        }
    }
}
