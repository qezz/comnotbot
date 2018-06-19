use std::collections::HashMap;

use futures::prelude::*;
use futures_retry::{RetryPolicy, StreamRetryExt};

use tokio_core::reactor::Core;
use telegram_bot::{Api, Error as TelegramError};
use telegram_bot::{MessageChat, UpdateKind};

use errors::*;
use db::ChatDb;

pub type BotResult<T> = Result<T, BotError>;

type ChatCache = HashMap<MessageChat, ChatDb>;

pub struct Bot {
    api: Api,
    core: Core,
    // chat_cache: ChatCache,
}

impl Bot {
    pub fn new(token: &str) -> BotResult<Bot> {
        let core = Core::new()?;
        let api = Api::configure(token).build(core.handle())?;
        Ok(Bot {
            api: api,
            core: core,
            // chat_cache: HashMap::new(),
        })
    }

    pub fn run(mut self) -> BotResult<()> {
        self.core.run(handle_updates(self.api))?;
        Ok(())
    }
}

#[async]
fn handle_updates(api: Api) -> Result<(), TelegramError> {
    let mut chat_cache: ChatCache = HashMap::new();
    #[async]
    for update in api.stream().retry(handle_update_error) {
        match update.kind {
            UpdateKind::Message(message)=> {
                println!("{:?}", message);
                let msg_chat = message.chat.clone();

                // match chat_cache.entry(msg_chat) {
                //     Vacant(o) => {
                //         o.insert(db::new(msg_chat));
                //     },
                //     _ => {},
                // }

                // match chat_cache.entry(msg_chat) {
                //     Occupied(o) => {
                //         o.insert(db::new(msg_chat));
                //     },
                //     _ => {},
                // }
                let chat_db = chat_cache.entry(msg_chat.clone()).or_insert(ChatDb::new(msg_chat));
                (*chat_db).append_raw(&format!("{:?}", message));
            },
            _ => { /* skip */ },
        }
    }

    Ok(())
}

fn handle_update_error(error: TelegramError) -> RetryPolicy<TelegramError> {
    println!("An error has occurred while getting update: {:?}", error);
    println!("Retrying");
    RetryPolicy::Repeat
}
