use teleborg;
use teleborg::Command;
use teleborg::objects::Update;

use std::collections::HashMap;

// use core::chat::Chat;
use core::db::ChatDb;

use lmdb;
use bincode;

// fn main_dispatcher(_bot: &teleborg::Bot, update: Update, _: Option<Vec<&str>>) {
// }

pub struct Bot {
    chats: HashMap<i64, ChatDb>,
    // dispatcher: Dispatcher::new(),
    // updater: Option<teleborg::Updater>,
}

// // TODO: implement teleborg::Command for CoreDispatcher
// // or simply for Bot
// struct CoreDispatcher {
// }

impl Bot {
    pub fn new(// token: String
    ) -> Bot {

        // let mut dsp = teleborg::Dispatcher::new();
        // dsp.add_message_handler();

        Bot {
            chats: HashMap::new(),
            // updater: None, // teleborg::Updater::start(Some(token), None, None, None, dsp)
        }
    }

    // pub fn start(&mut self, token: String) -> Result<(), ()> {
    //     let mut dsp = teleborg::Dispatcher::new();
    //     dsp.add_message_handler(main_dispatcher);

    //     self.updater = Some(teleborg::Updater::start(Some(token), None, None, None, dsp));
    //     Ok(())
    // }


    // TODO: Use failure
    fn write_to_chat_with_id(&mut self, chat_id: i64, bytes: &Vec<u8>) -> Result<(), lmdb::MdbError> {
        let mut the_chat = self.find_or_add(chat_id)?;

        // FIXME: Don't use unwrap
        let res = the_chat.append_raw(bytes);
        if let Err(e) = res {
            error!("Cannot write to db: {:?}", e);
        }

        Ok(())
    }

    fn find_or_add(&mut self, id: i64) -> Result<&mut ChatDb, lmdb::MdbError> {
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

// fn eventually_add(map: &mut HashMap<i32, A>, a: A) -> Result<&mut A, ()> {
//     match map.entry(0) {
//         Entry::Vacant(entry) => {
//             let a = A::new()?;
//             Ok(entry.insert(a))
//         }
//         Entry::Occupied(entry) => Ok(entry.into_mut()),
//     }
// }

impl Command for Bot {
    fn execute(&mut self, bot: &teleborg::Bot, update: Update, args: Option<Vec<&str>>) {
        let chat_id = update.message.clone().unwrap().chat.id;
        let bytes = &bincode::serialize(&update).unwrap();
        self.write_to_chat_with_id(chat_id, bytes);
    }
}
