use async_chat::utils::ChatResult;
use std::sync::Arc;

mod group;
mod grouptable;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server: ADDRESS");
    let chat_group_table = Arc::new(grouptable::GroupTable::new());

    async_std::task::block_on(async { todo!() });

    Ok(())
}
