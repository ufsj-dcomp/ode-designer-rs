use derive_more::From;
use imnodes::{InputPinId, NodeId, OutputPinId};

use crate::{core::app::Link, exprtree::ExpressionNode};

#[derive(Debug, Clone)]
pub struct SendData {
    pub data: ExpressionNode<InputPinId>,
    pub from_output: OutputPinId,
    pub to_input: InputPinId,
}

#[derive(Debug, Clone)]
pub enum Message {
    SendData(SendData),
    AddLink(Link),
    RemoveLink(Link),
    AttributeAssignerOperatesOn {
        assigner_id: NodeId,
        value: NodeId,
    },
    SetNodePos {
        node_id: NodeId,
        screen_space_pos: [f32; 2],
    },
}

#[derive(Debug, Clone)]
pub struct TaggedMessage {
    pub tag: usize,
    pub message: Message,
}

#[derive(Debug, Clone, From, Default)]
pub struct MessageQueue {
    pub messages: Vec<TaggedMessage>,
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
        self.messages.push(TaggedMessage {
            tag: self.current_tag,
            message,
        });
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
