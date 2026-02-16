use async_chat::utils::ChatResult;
use std::sync::Arc;
use tokio::sync::broadcast;

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn join(&self, outboud: Arc<Outbound>) {
        todo!()
    }

    pub fn post(&self, message: Arc<String>) {
        todo!()
    }
}
