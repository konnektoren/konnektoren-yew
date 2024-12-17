use serde::{Deserialize, Serialize};
use yew_chat::prelude::Message;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct Inbox {
    pub messages: Vec<Message>,
    pub read_messages: Option<Vec<String>>,
}

impl Inbox {
    pub fn merge(&mut self, other: &Inbox) {
        for message in &other.messages {
            if let Some(id) = &message.id {
                if !self.messages.iter().any(|m| m.id.as_ref() == Some(id)) {
                    self.messages.push(message.clone());
                }
            } else {
                self.messages.push(message.clone());
            }
        }

        if let Some(other_read) = &other.read_messages {
            let self_read = self.read_messages.get_or_insert_with(Vec::new);
            for read_id in other_read {
                if !self_read.contains(read_id) {
                    self_read.push(read_id.clone());
                }
            }
        }

        self.messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }

    pub fn mark_as_read(&mut self, message_id: String) {
        let read_messages = self.read_messages.get_or_insert_with(Vec::new);
        if !read_messages.contains(&message_id) {
            read_messages.push(message_id);
        }
    }

    pub fn add_uniquely(&mut self, message: Message) {
        if let Some(id) = &message.id {
            if !self.messages.iter().any(|m| m.id.as_ref() == Some(id)) {
                self.messages.push(message);
            }
        } else {
            self.messages.push(message);
        }
        self.messages.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use yew_chat::prelude::Message;

    #[test]
    fn default_inbox() {
        let inbox = Inbox {
            messages: vec![],
            read_messages: Some(vec![]),
        };
        assert!(inbox.messages.is_empty());
        assert!(inbox.read_messages.is_some());
    }

    #[test]
    fn merge_inbox() {
        let mut inbox = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec!["1".to_string()]),
        };
        let other = Inbox {
            messages: vec![Message {
                id: Some("2".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec!["2".to_string()]),
        };
        inbox.merge(&other);
        assert_eq!(inbox.messages.len(), 2);
        assert_eq!(inbox.read_messages.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn merge_inbox_no_duplicates() {
        let mut inbox = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec!["1".to_string()]),
        };
        let other = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec!["1".to_string()]),
        };
        inbox.merge(&other);
        assert_eq!(inbox.messages.len(), 1);
        assert_eq!(inbox.read_messages.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn mark_as_read() {
        let mut inbox = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec![]),
        };
        inbox.mark_as_read("1".to_string());
        assert_eq!(inbox.read_messages.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn add_uniquely() {
        let mut inbox = Inbox {
            messages: vec![Message {
                id: Some("1".to_string()),
                ..Default::default()
            }],
            read_messages: Some(vec![]),
        };
        inbox.add_uniquely(Message {
            id: Some("2".to_string()),
            ..Default::default()
        });
        assert_eq!(inbox.messages.len(), 2);
    }
}
