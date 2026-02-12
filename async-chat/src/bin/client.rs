use async_chat::{
    FromClient,
    utils::{self, ChatResult},
};
use async_std::{
    io::{self, BufReadExt, WriteExt},
    net,
    stream::StreamExt,
};

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!(
        "Commands:\n\
        join GROUP\n\
        post GROUP MESSAGE...\n\
        Type Control-D (on Unix) or Control-Z (on Windows) \
        to close the connection."
    );
    let mut command_lines = io::BufReader::new(io::stdin()).lines();
    while let Some(command_result) = command_lines.next().await {
        let command = command_result?;

        let request = match parse_command(&command) {
            Some(request) => request,
            None => continue,
        };
        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

fn parse_command(command: &str) -> Option<FromClient> {
    todo!()
}

fn main() {}
