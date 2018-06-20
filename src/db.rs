use lmdb;

/// DbInstance is per chat db
pub struct ChatDb {
    // chat_id: i64,
    chat_id: i64,
    current_unique_id: u64,

    env: lmdb::Environment,
    db_handle: lmdb::DbHandle,
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
}

#[cfg(test)]
mod tests {
    use telegram_bot as tg;
    use telegram_bot_raw as tg_raw;
    // use bincode;
    use lmdb;
    use serde_json;

    use std::char;

    fn unescape(s: &str) -> String {
        let mut result = String::with_capacity(s.len());
        let mut chars = s.chars();
        while let Some(ch) = chars.next() {
            result.push(
                if ch != '\\' {
                    ch
                } else {
                    match chars.next() {
                        Some('u') => {
                            let value = chars.by_ref().take(4).fold(0, |acc, c| acc * 16 + c.to_digit(16).unwrap());
                            char::from_u32(value).unwrap()
                        }
                        Some('b') => '\x08',
                        Some('f') => '\x0c',
                        Some('n') => '\n',
                        Some('r') => '\r',
                        Some('t') => '\t',
                        Some(ch) => ch,
                        _ => panic!("Malformed escape"),
                    }
                }
            )
        }
        result
    }

    #[test]
    fn core_logic() {
        let env = lmdb::EnvBuilder::new().open("tests-db", 0o777)
                                         .unwrap();
        let db_handle = env.get_default_db(lmdb::DbFlags::empty()).unwrap();

        let current_unique_id = 0;
        let bytes: &[u8] = &[123_u8, 34, 111, 107, 34, 58, 116, 114, 117, 101, 44, 34, 114, 101, 115, 117, 108, 116, 34, 58, 91, 123, 34, 117, 112, 100, 97, 116, 101, 95, 105, 100, 34, 58, 53, 56, 49, 49, 54, 52, 56, 49, 44, 10, 34, 109, 101, 115, 115, 97, 103, 101, 34, 58, 123, 34, 109, 101, 115, 115, 97, 103, 101, 95, 105, 100, 34, 58, 49, 49, 44, 34, 102, 114, 111, 109, 34, 58, 123, 34, 105, 100, 34, 58, 55, 57, 50, 57, 49, 50, 48, 44, 34, 105, 115, 95, 98, 111, 116, 34, 58, 102, 97, 108, 115, 101, 44, 34, 102, 105, 114, 115, 116, 95, 110, 97, 109, 101, 34, 58, 34, 83, 101, 114, 103, 101, 121, 34, 44, 34, 108, 97, 115, 116, 95, 110, 97, 109, 101, 34, 58, 34, 92, 117, 50, 55, 99, 53, 40, 92, 117, 51, 48, 99, 52, 41, 92, 117, 50, 55, 99, 54, 34, 44, 34, 117, 115, 101, 114, 110, 97, 109, 101, 34, 58, 34, 107, 101, 122, 118, 105, 115, 105, 111, 110, 34, 44, 34, 108, 97, 110, 103, 117, 97, 103, 101, 95, 99, 111, 100, 101, 34, 58, 34, 101, 110, 45, 82, 85, 34, 125, 44, 34, 99, 104, 97, 116, 34, 58, 123, 34, 105, 100, 34, 58, 55, 57, 50, 57, 49, 50, 48, 44, 34, 102, 105, 114, 115, 116, 95, 110, 97, 109, 101, 34, 58, 34, 83, 101, 114, 103, 101, 121, 34, 44, 34, 108, 97, 115, 116, 95, 110, 97, 109, 101, 34, 58, 34, 92, 117, 50, 55, 99, 53, 40, 92, 117, 51, 48, 99, 52, 41, 92, 117, 50, 55, 99, 54, 34, 44, 34, 117, 115, 101, 114, 110, 97, 109, 101, 34, 58, 34, 107, 101, 122, 118, 105, 115, 105, 111, 110, 34, 44, 34, 116, 121, 112, 101, 34, 58, 34, 112, 114, 105, 118, 97, 116, 101, 34, 125, 44, 34, 100, 97, 116, 101, 34, 58, 49, 53, 50, 57, 52, 53, 48, 51, 48, 54, 44, 34, 116, 101, 120, 116, 34, 58, 34, 104, 101, 121, 34, 125, 125, 93, 125];
        let _m = String::from_utf8_lossy(bytes);
        println!("raw:\n{}", _m);
        println!("raw:\n{:?}", _m);
        let _m = unescape(&_m);
        println!("raw:\n{}", _m);
        println!("raw:\n{:?}", _m);

        {
            let txn = env.new_transaction().unwrap();
            {
                let db = txn.bind(&db_handle);

                match db.set(&current_unique_id, &bytes) {
                    Ok(_) => {},
                    Err(_) => { panic!() },
                };
            }

            match txn.commit() {
                Ok(_) => {},
                Err(_) => { panic!() },
            };
        }

        // ---

        let reader = env.get_reader().unwrap();
        let db = reader.bind(&db_handle);
        let msg = db.get::<&str>(&current_unique_id).unwrap();
        println!("raw msg:\n{:?}", msg);

        // ---

        let msg = serde_json::from_str::<tg_raw::types::RawResponse<Vec<tg::Update>>>(msg);
        assert!(msg.is_ok());
        println!("msg:\n{:#?}", msg);
    }
}
