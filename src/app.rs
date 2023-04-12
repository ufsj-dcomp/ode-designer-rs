use std::collections::HashMap;

use eframe::epaint::ahash::HashMapExt;

use crate::nodes::{Node, NodeId, Pin, PinId, NodeClass, self};

use crate::message::{ Message, SendData };

pub type LinkId = i32;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: LinkId,
    pub input_pin_id: PinId,
    pub output_pin_id: PinId,
}

impl Link {
    pub fn new(input_pin_id: &PinId, output_pin_id: &PinId) -> Self { Self { id: nodes::next_id(), input_pin_id: *input_pin_id, output_pin_id: *output_pin_id } }
}

pub struct App {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) pins: HashMap<PinId, NodeId>,
    pub(crate) links: Vec<Link>,
    pub state: Option<AppState>,
}

pub enum AppState {
    AddingNode{ name: String, index: i32 },
}

impl App {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            pins: HashMap::new(),
            links: Vec::new(),
            state: None
        }
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

    #[must_use]
    pub fn add_link(&mut self, from_id: &PinId, to_id: &PinId) {
        dbg!(from_id, to_id);
        /* let from_node = (self.pins[from_id]);
        let to_node = self.get_node(self.pins[to_id]).unwrap();
        from_node.get_output_mut(from_id).unwrap().link_to(to_id);
        to_node.get_output_mut(to_id).unwrap().link_to(from_id); */
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_pin(&self, id: &PinId) -> Option<&Pin> {
        // Find the node that owns the pin
        let node_id = self.pins.get(&id)?;
        let node = self.nodes.get(node_id)?;
        // Search inputs first;
        if let Some(input) = node.get_input(id) {
            return Some(input)
        } else if let Some(output) = node.get_output(id) {
            return Some(output)
        }
        unreachable!("Bug: Node should contain pin")
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

    pub fn update_state(&mut self, message: Message) -> Vec<Message> {
        match message {
            Message::SendData(SendData {data, from_output, to_input}) => {
                let node_id = self.pins.get_mut(&to_input).unwrap();
                let node = self.nodes.get_mut(&node_id).unwrap();
                node.receive_data(&to_input, data.clone());
                node.send_data(data)
            }
        }
    }
}

