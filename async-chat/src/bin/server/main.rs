use async_chat::utils::ChatResult;
use async_std::{net, stream::StreamExt, task};
use connection::serve;
use std::sync::Arc;

mod connection;
mod group;
mod grouptable;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server: ADDRESS");
    let chat_group_table = Arc::new(grouptable::GroupTable::new());

    async_std::task::block_on(async {
        let listener = net::TcpListener::bind(address).await?;
        let mut new_connections = listener.incoming();

        while let Some(socket_result) = new_connections.next().await {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }

        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        eprintln!("Error: {}", error);
    }
}
