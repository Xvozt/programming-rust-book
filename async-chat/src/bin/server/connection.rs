use std::sync::Arc;

use async_chat::{
    FromClient, FromServer,
    utils::{self, ChatResult},
};
use async_std::{
    io::{BufReader, WriteExt},
    net::TcpStream,
    stream::StreamExt,
    sync::Mutex,
};

use crate::grouptable::GroupTable;

pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    let outboud = Arc::new(Outbound::new(socket.clone()));
    let buffered = BufReader::new(socket);
    let mut from_client = utils::recieve_as_json(buffered);
    while let Some(request_result) = from_client.next().await {
        let request = request_result?;

        let result = match request {
            FromClient::Join { group_name } => {
                let group = groups.get_or_create(group_name);
                group.join(outboud.clone());
                Ok(())
            }
            FromClient::Post {
                group_name,
                message,
            } => match groups.get(&group_name) {
                Some(group) => {
                    group.post(message);
                    Ok(())
                }
                None => Err(format!("Group '{}' doesn't exist", group_name)),
            },
        };
        if let Err(message) = result {
            let report = FromServer::Error(message);
            outboud.send(report).await?;
        }
    }

    Ok(())
}

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(to_client: TcpStream) -> Self {
        Outbound(Mutex::new(to_client))
    }

    pub async fn send(&self, packet: FromServer) -> ChatResult<()> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &packet).await?;
        guard.flush().await?;
        Ok(())
    }
}
