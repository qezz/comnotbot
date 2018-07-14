use teleborg;
use teleborg::Command;
use teleborg::objects::Update;

use std::collections::HashMap;

// use core::chat::Chat;
use core::db::ChatDb;
use core::errors::{BotError, BotErrorKind};

use failure::{Fail, ResultExt};

use lmdb;
use bincode;

pub struct Bot {
    chats: HashMap<i64, ChatDb>,
}

impl Bot {
    pub fn new(// token: String
    ) -> Bot {
        Bot {
            chats: HashMap::new(),
        }
    }

    // TODO: Use failure
    fn write_to_chat_with_id(&mut self, chat_id: i64, bytes: &Vec<u8>) -> Result<(), BotError> { // lmdb::MdbError> {
        let the_chat = self.find_or_add(chat_id)?; // .context(BotErrorKind::DbError)?;

        let res = the_chat.append_raw(bytes);
        if let Err(e) = res {
            error!("Cannot write to db: {:?}", e);
            Err(BotErrorKind::ChatWriteError{id: chat_id})?
        }

        Ok(())
    }

    fn find_or_add(&mut self, id: i64) -> Result<&mut ChatDb, BotError> { // lmdb::MdbError> {
        use std::collections::hash_map::Entry;

        match self.chats.entry(id) {
            Entry::Vacant(entry) => {
                let chat = ChatDb::new(id)?;
                Ok(entry.insert(chat))
            },
            Entry::Occupied(entry) => Ok(entry.into_mut()),
        }
    }

}

impl Command for Bot {
    fn execute(&mut self, _bot: &teleborg::Bot, update: Update, _args: Option<Vec<&str>>) {
        debug!("dispatcher: received an update");
        if let Some(ref m) = update.message {
            let chat_id = m.chat.id;
            debug!("dispatcher: serializing bytes");
            let bytes = &bincode::serialize(&update).unwrap();
            if let Err(e) = self.write_to_chat_with_id(chat_id, bytes) {
                error!("Error while writing to db: {:?}", e);
            }
        }
    }
}
