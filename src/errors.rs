use std::fmt;
use std::{error::Error, io::Error as IoError};

// use telegram_bot::prelude::*;
use telegram_bot::{Error as TelegramError};


#[derive(Debug)]
pub struct BotError {
    description: String,
}

impl BotError {
    fn unknown_command(cmd: &str) -> BotError {
        BotError {
            description: format!("Unknown command: {}", cmd),
        }
    }
}

impl From<IoError> for BotError {
    fn from(err: IoError) -> BotError {
        BotError {
            description: err.to_string(),
        }
    }
}

impl From<TelegramError> for BotError {
    fn from(err: TelegramError) -> BotError {
        BotError {
            description: err.to_string(),
        }
    }
}

impl Error for BotError {
    fn description(&self) -> &str {
        &self.description
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.description)
    }
}
