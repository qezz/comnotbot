use core::db;

#[derive(Debug)]
pub struct Chat {
    id: i64,
    db: Option<db::ChatDb>,
}

impl PartialEq for Chat {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Chat {
    // fn eq(&self, other: &Self) -> bool {
    //     self.id == other.id
    // }
}


impl Chat {
    /// Creates a Chat but doesn't make any side effects
    /// (e.g. creating a separated chat directory)
    pub fn with_id(id: i64) -> Chat {
        Chat {
            id: id,
            db: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_chat() {
        let chat_id = 321;

        let expected = Chat {
            id: chat_id,
            db: None,
        };

        assert_eq!(Chat::with_id(chat_id), expected);
    }

    // #[test]
    // fn chat_with_id() {
    // }
}
