extern crate comnotbot;

// use dotenv::dotenv;
// config-rs
use std::env;
use comnotbot::Bot;

#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate lmdb_rs as lmdb;

fn main() {
    let token = env::var("COMNOTBOT_TOKEN").expect("Can't get token from environment");
    let app = Bot::new(&token).expect("Failed to create the bot");
    app.run().expect("Failed to run the bot");
}
