use std::collections::{HashMap, HashSet};

use crate::nodes::{self, Node, NodeClass, NodeId, Pin, PinId};

use crate::message::{Message, MessageQueue, SendData, TaggedMessage};

pub type LinkId = i32;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: LinkId,
    pub input_pin_id: PinId,
    pub output_pin_id: PinId,
}

impl Link {
    pub fn new(input_pin_id: &PinId, output_pin_id: &PinId) -> Self {
        Self {
            id: nodes::next_id(),
            input_pin_id: *input_pin_id,
            output_pin_id: *output_pin_id,
        }
    }
}

#[derive(Default)]
pub struct App {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) pins: HashMap<PinId, NodeId>,
    pub(crate) links: Vec<Link>,
    pub state: Option<AppState>,
    pub messages: MessageQueue,
    pub received_messages: HashMap<NodeId, HashSet<usize>>,
}

pub enum AppState {
    AddingNode { name: String, index: usize },
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Node) {
        let node_id = *node.id();
        for input in node.inputs() {
            self.pins.insert(*input.id(), node_id);
        }
        for output in node.outputs() {
            self.pins.insert(*output.id(), node_id);
        }
        self.nodes.insert(node_id, node);
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_pin(&self, id: &PinId) -> Option<&Pin> {
        // Find the node that owns the pin
        let node_id = self.pins.get(&id)?;
        let node = self.nodes.get(node_id)?;
        node.get_pin(id)
    }

    pub fn get_pin_mut(&mut self, id: &PinId) -> Option<&mut Pin> {
        // Find the node that owns the pin
        let node_id = self.pins.get(&id)?;
        let node = self.nodes.get_mut(node_id)?;
        node.get_pin_mut(id)
    }

    pub fn get_link(&self, input_id: &PinId) -> Option<&Link> {
        self.links
            .iter()
            .find(|link| link.input_pin_id == *input_id)
    }

    pub fn remove_node(&mut self, id: &NodeId) -> Option<Node> {
        let node = self.nodes.remove(id)?;
        for input in node.inputs() {
            self.pins.remove(input.id());
        }
        for output in node.outputs() {
            self.pins.remove(&output.id());
        }
        Some(node)
    }

    fn handle_message(&mut self, tagged: TaggedMessage) -> Vec<Message> {
        match tagged.message {
            Message::SendData(SendData {
                data,
                from_output,
                to_input,
            }) => {
                let node_id = self.pins.get_mut(&to_input).unwrap();
                let node = self.nodes.get_mut(&node_id).unwrap();
                let input_pin = node.get_input(&to_input).unwrap();
                let received_msgs = self.received_messages.entry(*node_id).or_default();
                if received_msgs.contains(&tagged.tag) {
                    return vec![];
                }
                received_msgs.insert(tagged.tag);
                node.receive_data(&to_input, data.clone())
            }
            Message::AddLink(link) => {
                if self.get_link(&link.input_pin_id).is_some() {
                    return vec![];
                }
                let result: Option<Vec<Message>> = try {
                    let Link {
                        input_pin_id,
                        output_pin_id,
                        ..
                    } = &link;
                    let node_ids = [self.pins.get(input_pin_id)?, self.pins.get(output_pin_id)?];
                    let [input_node, output_node] = self.nodes.get_many_mut(node_ids)?;
                    if !input_node.should_link(input_pin_id) {
                        // Poor man's early return
                        None?
                    }
                    input_node
                        .get_input_mut(input_pin_id)?
                        .link_to(output_pin_id);
                    output_node
                        .get_output_mut(output_pin_id)?
                        .link_to(input_pin_id);
                    self.links.push(link);
                    output_node.send_data()
                };
                result.unwrap()
            }
        }
    }

    pub fn add_link(&mut self, start_pin: &PinId, end_pin: &PinId) {
        self.messages.push(Link::new(end_pin, start_pin).into());
    }

    pub fn update(&mut self) {
        let mut new_messages = MessageQueue::with_tag(self.messages.current_tag());
        for tagged in std::mem::take(&mut self.messages) {
            let tag = tagged.tag;
            let newmsgs = self.handle_message(tagged);
            for newmsg in newmsgs {
                new_messages.push_tagged(newmsg, tag);
            }
        }
        self.messages = new_messages;
    }
}

/* impl App {
    pub fn as_model(&self) -> odeir::Model {
        let nodes = self
            .nodes
            .iter()
            .filter_map(|(id, n)| Some((*id as u32, n.as_odeir_node()?)))
            .collect();
        let constants = self
            .nodes
            .iter()
            .filter_map(|(_, n)| Some(n.as_odeir_constant()?))
            .collect();
        odeir::Model {
            nodes,
            constants,
            // !TODO: implement medatada
            meta_data: odeir::MetaData::default(),
        }
    }
} */
