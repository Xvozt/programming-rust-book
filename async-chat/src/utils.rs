use async_std::prelude::*;
use std::error::Error;

use async_std::io::WriteExt;
use serde::{Serialize, de::DeserializeOwned};

pub type ChatError = Box<dyn Error + Send + Sync + 'static>;
pub type ChatResult<T> = Result<T, ChatError>;

pub async fn send_as_json<S, P>(outbound: &mut S, packet: &P) -> ChatResult<()>
where
    S: async_std::io::Write + Unpin,
    P: Serialize,
{
    let mut json = serde_json::to_string(&packet)?;
    json.push('\n');
    outbound.write_all(json.as_bytes()).await?;
    Ok(())
}

pub fn recieve_as_json<S, P>(inboud: S) -> impl Stream<Item = ChatResult<P>>
where
    S: async_std::io::BufRead + Unpin,
    P: DeserializeOwned,
{
    inboud.lines().map(|line_res| -> ChatResult<P> {
        let line = line_res?;
        let parsed = serde_json::from_str::<P>(&line)?;
        Ok(parsed)
    })
}
