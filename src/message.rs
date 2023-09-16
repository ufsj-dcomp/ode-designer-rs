
use derive_more::From;

use crate::{nodes::{Data, PinId}, app::Link};

#[derive(Debug, Clone)]
pub struct SendData {
    pub data: Data,
    pub from_output: PinId,
    pub to_input: PinId,
}

#[derive(Debug, Clone, From)]
pub enum Message {
    SendData(SendData),
    AddLink(Link),
}

#[derive(Debug, Clone)]
pub struct TaggedMessage {
    pub tag: usize,
    pub message: Message,
}

#[derive(Debug, Clone, From, Default)]
pub struct MessageQueue {
    messages: Vec<TaggedMessage>,
    current_tag: usize,
}

impl MessageQueue {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tag(tag: usize) -> Self {
        Self {
            messages: vec![],
            current_tag: tag,
        }
    }

    pub fn current_tag(&self) -> usize {
        self.current_tag
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(TaggedMessage { tag: self.current_tag, message });
        self.current_tag += 1;
    }

    pub fn push_tagged(&mut self, message: Message, tag: usize) {
        self.messages.push(TaggedMessage { tag, message });
    }

    pub fn iter(&self) -> impl Iterator<Item = &TaggedMessage> {
        self.messages.iter()
    }
}

impl IntoIterator for MessageQueue {
    type Item = TaggedMessage;
    type IntoIter = std::vec::IntoIter<TaggedMessage>;
    fn into_iter(self) -> std::vec::IntoIter<TaggedMessage> {
        self.messages.into_iter()
    }
}
