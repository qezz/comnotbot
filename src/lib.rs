// #![feature(proc_macro, generators)]
// extern crate futures_await as futures;
// extern crate futures_retry;
// extern crate telegram_bot;
// extern crate telegram_bot_raw;
// extern crate tokio_core;

extern crate lmdb_rs as lmdb;
extern crate bincode;
extern crate serde;
extern crate serde_json;

extern crate teleborg;

pub mod bot;
// mod errors;
pub mod db;

// pub use self::bot;
pub use self::bot::{Bot};
