use futures::prelude::*;
use futures_retry::{RetryPolicy, StreamRetryExt};

use tokio_core::reactor::Core;
use telegram_bot::{Api, Error as TelegramError};

use errors::*;

pub type BotResult<T> = Result<T, BotError>;

pub struct Bot {
    api: Api,
    core: Core,
}

impl Bot {
    pub fn new(token: &str) -> BotResult<Bot> {
        let core = Core::new()?;
        let api = Api::configure(token).build(core.handle())?;
        Ok(Bot {
            api: api,
            core: core,
        })
    }

    pub fn run(mut self) -> BotResult<()> {
        self.core.run(handle_updates(self.api))?;
        Ok(())
    }
}

#[async]
fn handle_updates(api: Api) -> Result<(), TelegramError> {
    #[async]
    for update in api.stream().retry(handle_update_error) {
        match update.kind {
            any => {
                println!("{:?}", any);
            }
        }
    }

    Ok(())
}

fn handle_update_error(error: TelegramError) -> RetryPolicy<TelegramError> {
    println!("An error has occurred while getting update: {:?}", error);
    println!("Retrying");
    RetryPolicy::Repeat
}
