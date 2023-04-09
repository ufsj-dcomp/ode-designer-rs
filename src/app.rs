use std::collections::HashMap;

use crate::nodes::{Node, NodeId, Pin, PinId};

pub struct App {
    pub(crate) nodes: HashMap<NodeId, Node>,
    pub(crate) links: HashMap<PinId, NodeId>,
}

impl App {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            links: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, node: Node) {
        self.nodes.insert(node.id, node);

    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_pin(&self, id: PinId) -> Option<&Pin> {
        // Find the node that owns the pin
        let node_id = self.links.get(&id)?;
        let node = self.nodes.get(node_id)?;
        // Return the pin from the node
        Some(node.inputs.iter().find(|input| input.id == id).expect("Bug: Node should contain pin"))
    }

}

