use std::collections::HashMap;

use ::core::db::ChatDb;

use bincode;
use teleborg::objects::Update;

struct Summary {
    pub table: HashMap<String, i64>,
}

impl Summary {
    pub fn new() -> Summary {
        Summary {
            table: HashMap::new(),
        }
    }

    pub fn summarize(chat_db: &ChatDb) -> Summary {
        let iter = chat_db.iter();

        let mut s = Summary::new();

        for binary_item in iter {
            let item: Update = bincode::deserialize(&binary_item).unwrap();
            // println!("item: {:?}", );

            if let Some(m) = item.message {
                println!("{:?}", m.date);
                if let Some(u) = m.from {
                    // FIXME: use the whole user information instead of username
                    if let Some(un) = u.username {
                        let count = s.table.entry(un).or_insert_with(|| 0);
                        *count += 1;
                    } else {
                        let count = s.table.entry(format!("id {}", u.id).to_string()).or_insert_with(|| 0);
                        *count += 1;
                    }
                }
            }
        }

        s
    }
}


#[cfg(test)]
mod tests {
    extern crate bincode;

    use super::*;
    #[test]
    fn rsjk() {
        let id = -1001329862200;
        let chat_db = ChatDb::new(id).unwrap();

        let summary = Summary::summarize(&chat_db);

        for (k, v) in summary.table {
            println!("> {}: {}", k, v);
        }
    }
}
