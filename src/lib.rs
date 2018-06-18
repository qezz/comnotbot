#![feature(proc_macro, generators)]
extern crate futures_await as futures;
extern crate futures_retry;
extern crate telegram_bot;
extern crate tokio_core;

mod bot;
mod errors;

pub use self::bot::{Bot};
