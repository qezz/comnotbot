extern crate comnotbot;

extern crate serde_json;
extern crate bincode;
extern crate lmdb_rs as lmdb;
#[macro_use]
extern crate lazy_static;

extern crate teleborg;
extern crate config;

use teleborg::{Dispatcher, // Bot,
               Updater};
use teleborg::objects::Update;

use std::collections::HashMap;
use std::sync::Mutex;

// use comnotbot::bot;
use comnotbot::core::db::ChatDb;
use comnotbot::core::bot::Bot;

// static mut chat_cache: Option<HashMap<i64, ChatDb>> = None;

// lazy_static! {
//     static ref CHAT_CACHE: Mutex<HashMap<i64, ChatDb>> = {
//         let mut m = HashMap::new();
//         Mutex::new(m)
//     };
// }

#[macro_use] extern crate log;
#[macro_use] extern crate env_logger;


fn main() {
    env_logger::init();

    let mut settings = config::Config::default();
    settings.merge(config::Environment::with_prefix("COMNOTBOT")).unwrap();

    let token = settings.get::<String>("token")
        .expect("You must provide telegram bot token.");

    let mut dispatcher = Dispatcher::new();
    let mut cnb = Bot::new();
    dispatcher.add_message_handler(cnb);

    // Start the updater, the Updater will start the threads, one of which will poll for updates
    // and send those to the Dispatcher's thread which will act upon it with the registered handlers
    Updater::start(Some(token), None, None, None, dispatcher);
}

// Our first command handler
// fn test(_bot: &Bot, update: Update, _: Option<Vec<&str>>) {
//     // println!("update: {:?}", serde_json::to_string(&update));
//     // println!("update: {}", serde_json::to_string(&update).unwrap());
//     // println!("update: {:?}", bincode::serialize(&update));

//     let chat_id = update.message.clone().unwrap().chat.id;
//     let mut map = CHAT_CACHE.lock().unwrap();
//     let chat_db = map.entry(chat_id)
//                      .or_insert(
//                          {
//                              ChatDb::new(chat_id).unwrap_or(
//                              {
//                              error!("cannot create chat with id {}, skipping the message", chat_id);

//                              })
//                          }
//     );
//     // match chat_db {
//     //     Occupied(occupied) => {

//     //     },
//     //     Vacant(vacant) => {

//     //     },
//     // }

//     let res = (*chat_db).append_raw(&bincode::serialize(&update).unwrap());
//     if let Err(e) = res {
//         println!("Cannot write to db: {:?}", e);
//     }
// }
