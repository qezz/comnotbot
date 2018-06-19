#![feature(proc_macro, generators)]
extern crate futures_await as futures;
extern crate futures_retry;
extern crate telegram_bot;
extern crate tokio_core;

extern crate lmdb_rs as lmdb;
extern crate bincode;

mod bot;
mod errors;
mod db;

pub use self::bot::{Bot};
