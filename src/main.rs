extern crate comnotbot;

extern crate serde_json;
extern crate bincode;
extern crate lmdb_rs as lmdb;

extern crate teleborg;
extern crate config;

use teleborg::{Dispatcher, Updater};

// use comnotbot::bot;
use comnotbot::core::bot::Bot;

#[macro_use] extern crate log;
extern crate env_logger;


fn main() {
    env_logger::init();

    let mut settings = config::Config::default();
    settings.merge(config::Environment::with_prefix("COMNOTBOT")).unwrap();

    let token = settings.get::<String>("token")
        .expect("You must provide telegram bot token.");

    let mut dispatcher = Dispatcher::new();
    let cnb = Bot::new();
    dispatcher.add_message_handler(cnb);

    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(Some(token), None, None, None, dispatcher);
}
