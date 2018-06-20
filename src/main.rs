extern crate comnotbot;

// use dotenv::dotenv;
// config-rs
use std::env;
// use comnotbot::Bot;

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate lmdb_rs as lmdb;

extern crate teleborg;
extern crate config;

use teleborg::{Dispatcher, Bot, Updater};
use teleborg::objects::Update;

use std::collections::HashMap;

fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::Environment::with_prefix("COMNOTBOT")).unwrap();

    let token = settings.get::<String>("token").expect("You must provide telegram bot token.");
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_message_handler(test);

    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(Some(token), None, None, None, dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    println!("update: {:?}", update);
    bot.reply_to_message(&update, "It works!").unwrap();
}
