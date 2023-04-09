use std::collections::HashMap;

use crate::nodes::{Node, NodeId, Pin, PinId, NodeClass};

use crate::message::{ Message, SendData };


pub struct App {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) links: HashMap<PinId, NodeId>,
    pub state: Option<AppState>,
}

pub enum AppState {
    AddingNode{ name: String, index: i32 },
}

impl App {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
            state: None
        }
    }

    pub fn add_node(&mut self, node: Node) {
        let node_id = *node.id();
        for input in node.inputs() {
            self.links.insert(*input.id(), node_id);
        }
        for output in node.outputs() {
            self.links.insert(*output.id(), node_id);
        }
        self.nodes.insert(*node.id(), node);
    }

    fn update_pin(&mut self, new_pin: Pin) -> bool {
        if let Some(pin) = self.get_pin_mut(new_pin.id()) {
            *pin = new_pin;
            return true
        }
        false
    }

    pub fn link_pin(&mut self, from_id: &PinId, to_id: &PinId) -> bool {
        let new_from = match self.get_pin(from_id) {
            Some(from) => from.link_to(to_id),
            None => return false,
        };
        let new_to = match self.get_pin(to_id) {
            Some(to) => to.link_to(from_id),
            None => return false,
        };
        self.update_pin(new_from) &&
        self.update_pin(new_to)
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_pin_mut(&mut self, id: &PinId) -> Option<&mut Pin> {
        // Find the node that owns the pin
        let node_id = self.links.get_mut(&id)?;
        let node = self.nodes.get_mut(node_id)?;
        // Return the pin from the node
        Some(node.inputs_mut().iter_mut().find(|input| input.id() == id).expect("Bug: Node should contain pin"))
    }
    pub fn get_pin(&self, id: &PinId) -> Option<&Pin> {
        // Find the node that owns the pin
        let node_id = self.links.get(&id)?;
        let node = self.nodes.get(node_id)?;
        // Return the pin from the node
        Some(node.inputs().iter().find(|input| input.id() == id).expect("Bug: Node should contain pin"))
    }

    pub fn remove_node(&mut self, id: &NodeId) -> Option<Node> {
        let node = self.nodes.remove(id)?;
        for input in node.inputs() {
            self.links.remove(input.id());
        }
        for output in node.outputs() {
            self.links.remove(&output.id());
        }
        Some(node)
    }

    pub fn update_state(&mut self, message: Message) -> Vec<Message> {
        match message {
            Message::SendData(SendData {data, from_output, to_input}) => {
                let node_id = self.links.get_mut(&to_input).unwrap();
                let node = self.nodes.get_mut(&node_id).unwrap();
                node.send_data(data)
            }
        }
    }
}

