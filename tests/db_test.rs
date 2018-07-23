// extern crate comnotbot;
// extern crate teleborg;
// extern crate serde_json;
// extern crate bincode;
// extern crate lmdb_rs as lmdb;

// use comnotbot::db::ChatDb;
// use teleborg::objects::Update;

// fn sample_update() -> Update {
//     let raw_json = "{\"update_id\":58116487,\"message\":{\"message_id\":20,\"from\":{\"id\":7929120,\"first_name\":\"Sergey\",\"last_name\":\"⟅(ツ)⟆\",\"username\":\"kezvision\",\"language_code\":\"en-RU\"},\"date\":1529530257,\"chat\":{\"id\":7929120,\"type\":\"private\",\"title\":null,\"username\":\"kezvision\",\"first_name\":\"Sergey\",\"last_name\":\"⟅(ツ)⟆\",\"all_members_are_administrators\":null},\"forward_from\":null,\"forward_from_chat\":null,\"forward_from_message_id\":null,\"forward_date\":null,\"reply_to_message\":null,\"edit_date\":null,\"text\":\"hey\",\"entities\":null,\"audio\":null,\"document\":null,\"game\":null,\"photo\":null,\"sticker\":null,\"video\":null,\"voice\":null,\"caption\":null,\"contact\":null,\"location\":null,\"venue\":null,\"new_chat_member\":null,\"left_chat_member\":null,\"new_chat_title\":null,\"new_chat_photo\":null,\"delete_chat_photo\":null,\"group_chat_created\":null,\"supergroup_chat_created\":null,\"channel_chat_created\":null,\"migrate_to_chat_id\":null,\"migrate_from_chat_id\":null,\"pinned_message\":null},\"edited_message\":null,\"inline_query\":null,\"chosen_inline_result\":null,\"callback_query\":null}";
//     let update: Update = serde_json::from_str(&raw_json).unwrap();
//     update
// }

// fn sample_raw_update() -> &'static str {
//     r#"{"update_id":58116487,"message":{"message_id":20,"from":{"id":7929120,"first_name":"Sergey","last_name":"⟅(ツ)⟆","username":"kezvision","language_code":"en-RU"},"date":1529530257,"chat":{"id":7929120,"type":"private","title":null,"username":"kezvision","first_name":"Sergey","last_name":"⟅(ツ)⟆","all_members_are_administrators":null},"forward_from":null,"forward_from_chat":null,"forward_from_message_id":null,"forward_date":null,"reply_to_message":null,"edit_date":null,"text":"hey","entities":null,"audio":null,"document":null,"game":null,"photo":null,"sticker":null,"video":null,"voice":null,"caption":null,"contact":null,"location":null,"venue":null,"new_chat_member":null,"left_chat_member":null,"new_chat_title":null,"new_chat_photo":null,"delete_chat_photo":null,"group_chat_created":null,"supergroup_chat_created":null,"channel_chat_created":null,"migrate_to_chat_id":null,"migrate_from_chat_id":null,"pinned_message":null},"edited_message":null,"inline_query":null,"chosen_inline_result":null,"callback_query":null}"#
// }

// #[test]
// fn bincode_message_serialization() {
//     let raw_update = sample_raw_update().as_bytes();
//     println!("raw: {:?}", raw_update);

//     let unser: Update = serde_json::from_str(raw_update).unwrap();

//     println!("unser: {:?}", unser);
//     let ser = bincode::serialize(&unser).unwrap();
//     println!("ser: {:?}", ser);

//     assert_eq!(raw_update, &ser[..]);
// }


///////////


// #[test]
// fn db_write_read() {
//     // FIXME: Tests shouldn't depend on chat id
//     let mut chat_db = ChatDb::new(0).unwrap();
//     let update = sample_update();

//     println!("ser");
//     let bin = bincode::serialize(&update).unwrap();

//     println!("append");
//     chat_db.append_raw(&bin).unwrap();

//     println!("start iterations");

//     let mut i = 0;
//     loop {
//         match chat_db.get(i) {
//             Some(ref x) => {
//                 println!("It's {:?}", bincode::deserialize::<Update>(&x));
//                 i += 1;
//             },
//             None => break,
//         }
//     }

//     println!("start iterations 2");
//     for x in chat_db.iter() {
//         println!("It's {:?}", bincode::deserialize::<Update>(&x));
//     }
// }
