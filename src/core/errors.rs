use failure::{Fail, Backtrace, Context};
use lmdb;

use std::fmt;

#[derive(Debug)]
pub struct BotError {
    inner: Context<BotErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum BotErrorKind {
    #[fail(display = "Can't find or create a chat with id {}", id)]
    ChatError {
        id: i64
    },

    #[fail(display = "Can't write to chat with id {}", id)]
    ChatWriteError {
        id: i64
    },

    #[fail(display = "A contextual error message.")]
    LmdbError, // (lmdb::MdbError),

    #[fail(display = "A contextual error message 2.")]
    ProcessingError,

    #[fail(display = "A contextual error message 3.")]
    Unknown,
}

impl Fail for BotError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl fmt::Display for BotError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.inner, f)
    }
}

impl BotError {
    pub fn kind(&self) -> BotErrorKind {
        *self.inner.get_context()
    }
}

impl From<BotErrorKind> for BotError {
    fn from(kind: BotErrorKind) -> BotError {
        BotError { inner: Context::new(kind) }
    }
}

impl From<Context<BotErrorKind>> for BotError {
    fn from(inner: Context<BotErrorKind>) -> BotError {
        BotError { inner: inner }
    }
}

// impl From<lmdb::MdbError> for BotError {
//     fn from(kind: lmdb::MdbError) -> BotError {
//         BotError { inner: Context::new(kind) }
//     }
// }

impl From<lmdb::MdbError> for BotError {
    fn from(kind: lmdb::MdbError) -> BotError {
        BotError {
            inner: Context::new(
                BotErrorKind::LmdbError // (kind)
            )
        }
    }
}
