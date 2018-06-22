extern crate comnotbot;

extern crate serde_json;
extern crate bincode;
extern crate lmdb_rs as lmdb;
#[macro_use]
extern crate lazy_static;

extern crate teleborg;
extern crate config;

use teleborg::{Dispatcher, Bot, Updater};
use teleborg::objects::Update;

use std::collections::HashMap;
use std::sync::Mutex;

// use comnotbot::bot;
use comnotbot::db::ChatDb;

// static mut chat_cache: Option<HashMap<i64, ChatDb>> = None;

lazy_static! {
    static ref CHAT_CACHE: Mutex<HashMap<i64, ChatDb>> = {
        let mut m = HashMap::new();
        Mutex::new(m)
    };
}


fn main() {
    let mut settings = config::Config::default();
    settings.merge(config::Environment::with_prefix("COMNOTBOT")).unwrap();

    let token = settings.get::<String>("token")
        .expect("You must provide telegram bot token.");
    let mut dispatcher = Dispatcher::new();
    dispatcher.add_message_handler(test);

    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(Some(token), None, None, None, dispatcher);
}

// Our first command handler
fn test(bot: &Bot, update: Update, _: Option<Vec<&str>>) {
    println!("update: {:?}", serde_json::to_string(&update));
    println!("update: {}", serde_json::to_string(&update).unwrap());
    println!("update: {:?}", bincode::serialize(&update));
    bot.reply_to_message(&update, "It works!").unwrap();

    let chat_id = (update.message.clone().unwrap().chat.id).clone();
    let mut map = CHAT_CACHE.lock().unwrap();
    let chat_db = map .entry(chat_id)
                      .or_insert(ChatDb::new(chat_id));
    (*chat_db).append_raw(&bincode::serialize(&update).unwrap());
}
