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

        // let cdb = ChatDb {
        //     chat_id: id,
        //     current_unique_id: 0,
        //     env: env,
        //     db_handle: db_handle,
        //     db: None,
        // };

        // let reader = {
        //     cdb.env.get_reader().unwrap();
        // }
        // let db = reader.bind(&db_handle);

        // cdb;

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

    // pub fn append_msg(&self, msg: &tg::Message) -> Result<(), ()> {
    //     let bytes = bincode::serialize(msg);

    //     self.append_raw(bytes)
    // }

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

    pub fn iter(&self) -> ChatDbIter { // MdbResult<CursorIterator<CursorIter>> {
        // let reader = self.env.get_reader().unwrap();
        // let db = reader.bind(&self.db_handle);
        // self.db.iter()
        ChatDbIter {
            chat_db: self,
            id: 0,
        }
    }

    // pub fn iter(&self) -> ChatDbIter {
    //     ChatDbIter {
    //         db: self.env.get_reader().and_then(|r| Ok(r.bind(&self.db_handle))).unwrap(),
    //         id: 0,
    //     }
    // }
}

pub struct ChatDbIter<'a> {
    chat_db: &'a ChatDb,
    // reader: Option<lmdb::ReadonlyTransaction<'a>>,
    // db: Option<lmdb::Database<'a>>,
    id: i64,
}

impl<'a> Iterator for ChatDbIter<'a> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {

        // let db = match self.db {
        //     None => {
        //         self.reader = Some(self.chat_db.env.get_reader().unwrap());
        //         Some(&self.reader.unwrap().bind(&self.chat_db.db_handle))
        //     }
        //     Some(db) => Some(db),
        // };
        // let db = db.unwrap();
        // let val = db.get::<Vec<u8>>(&self.id);
        // self.id += 1;
        // val.ok()

        // let val = self.db.get::<Vec<u8>>(&self.id);
        // self.id += 1;
        // val.ok()

        //
        // let result = self.chat_db.db.iter();

        //
        let r = self.chat_db.get(self.id);
        // let result = match r {
        //     Ok(v) => Some(v),
        //     Err(_) => None,
        // };
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
