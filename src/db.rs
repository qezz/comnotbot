use telegram_bot as tg;
// use bincode;
use lmdb;

/// DbInstance is per chat db
pub struct ChatDb {
    // chat_id: i64,
    chat_id: tg::ChatId,
    current_unique_id: u64,

    env: lmdb::Environment,
    db_handle: lmdb::DbHandle,
}

impl ChatDb {
    pub fn new(msg_chat: tg::types::MessageChat) -> ChatDb {
        let id = msg_chat.id();
        let env = lmdb::EnvBuilder::new().open(format!("chat_{:?}", id), 0o777)
                                   .unwrap();

        let db_handle = env.get_default_db(lmdb::DbFlags::empty()).unwrap();

        ChatDb {
            chat_id: id,
            current_unique_id: 0,
            env: env,
            db_handle: db_handle,
        }
    }

    pub fn append_raw(&mut self, bytes: &str) -> Result<(), ()> {
        {
            let txn = self.env.new_transaction().unwrap();
            {
                let db = txn.bind(&self.db_handle);

                match db.set(&self.current_unique_id, &bytes) {
                    Ok(_) => {},
                    Err(_) => return Err(())
                };
            }

            match txn.commit() {
                Ok(_) => {},
                Err(_) => return Err(())
            };
        }
        self.inc();
        Ok(())
    }

    // pub fn append_msg(&self, msg: &tg::Message) -> Result<(), ()> {
    //     let bytes = bincode::serialize(msg);

    //     self.append_raw(bytes)
    // }

    fn inc(&mut self) -> u64 {
        let cuid = self.current_unique_id;
        self.current_unique_id += 1;
        cuid
    }
}

#[cfg(test)]
mod tests {
}
