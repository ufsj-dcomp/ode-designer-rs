use imgui::Ui;
use imnodes::NodeId;
use linkme::distributed_slice;

use crate::{
    imgui::app::input_num,
    nodes::{Data, Node},
    pins::{OutputPin, Pin}, register_node,
};

use super::{
    NameAndConstructor, NodeSpecialization, NodeSpecializationInitializer, NODE_SPECIALIZATIONS,
};

register_node!(Population);

#[derive(Debug)]
pub struct Population {
    node: Node,
    initial_value: f64,
    output: OutputPin,
}

impl NodeSpecialization for Population {
    fn id(&self) -> NodeId {
        self.node.id()
    }

    fn name(&self) -> &str {
        &self.node.name
    }

    fn send_data(&self) -> Data {
        Data::Text(self.node.name.clone())
    }

    fn draw(&mut self, ui: &Ui) -> bool {
        ui.text("Initial Value: ");
        ui.same_line();
        input_num(ui, "##population initial value", &mut self.initial_value)
    }

    fn outputs(&self) -> Option<&[OutputPin]> {
        Some(std::array::from_ref(&self.output))
    }

    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        Some(std::array::from_mut(&mut self.output))
    }
}

impl NodeSpecializationInitializer for Population {
    fn new(node: Node) -> Self {
        let node_id = node.id();
        Self {
            node,
            initial_value: 0.00,
            output: Pin::new(node_id),
        }
    }
}
