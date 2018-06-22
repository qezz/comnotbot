use lmdb;

/// DbInstance is per chat db
pub struct ChatDb {
    chat_id: i64,
    current_unique_id: u64,

    env: lmdb::Environment,
    db_handle: lmdb::DbHandle,
    // db: Option<lmdb::Database<'a>>,
}

impl ChatDb {
    pub fn new(msg_chat: i64) -> ChatDb {
        let id = msg_chat;
        let env = lmdb::EnvBuilder::new().open(format!("chat_{:?}", id), 0o777)
                                   .unwrap();

        let db_handle = env.get_default_db(lmdb::DbFlags::empty()).unwrap();

        ChatDb {
            chat_id: id,
            current_unique_id: 0,
            env: env,
            db_handle: db_handle,
            // db: None,
        }
    }

    pub fn append_raw(&mut self, bytes: &Vec<u8>) -> Result<(), ()> {
        {
            let txn = self.env.new_transaction().unwrap();
            {
                let db = txn.bind(&self.db_handle);

                match db.set(&self.current_unique_id, bytes) {
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

    fn inc(&mut self) -> u64 {
        let cuid = self.current_unique_id;
        self.current_unique_id += 1;
        cuid
    }

    pub fn get(&self, id: i64) -> Option<Vec<u8>> {
        let reader = self.env.get_reader().unwrap();
        let db = reader.bind(&self.db_handle);
        db.get::<Vec<u8>>(&id).ok()
    }

    pub fn iter(&self) -> ChatDbIter {
        ChatDbIter {
            chat_db: self,
            id: 0,
        }
    }
}

pub struct ChatDbIter<'a> {
    chat_db: &'a ChatDb,
    id: i64,
}

impl<'a> Iterator for ChatDbIter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        // FIXME: ChatDb::get() creates a new transaction on every request.
        // In future, it may be the performance issue.
        let r = self.chat_db.get(self.id);

        self.id += 1;

        r
    }
}

#[cfg(test)]
mod tests {
    extern crate bincode;

    use super::*;

    #[test]
    fn iterates() {
        let mut chat_db = ChatDb::new(1);

        for i in 0..10 {
            let bin = bincode::serialize::<i32>(&i).unwrap();
            chat_db.append_raw(&bin).unwrap();
        }

        for (i, x) in chat_db.iter().enumerate() {
            assert_eq!(i as i32, bincode::deserialize::<i32>(&x).unwrap());
        }
    }
}
