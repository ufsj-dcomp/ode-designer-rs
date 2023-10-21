use imnodes::NodeId;
use linkme::distributed_slice;

use crate::{
    imgui::app::input_num,
    nodes::{Data, Node},
    pins::{OutputPin, Pin}, declare_node,
};

use super::{
    NameAndConstructor, NodeSpecialization, NodeSpecializationInitializer, NODE_SPECIALIZATIONS
};

declare_node!(Constant);

#[derive(Debug)]
pub struct Constant {
    node: Node,
    value: f64,
    output: OutputPin,
}

impl NodeSpecialization for Constant {
    fn id(&self) -> NodeId {
        self.node.id()
    }

    fn name(&self) -> &str {
        &self.node.name
    }

    fn send_data(&self) -> Data {
        Data::Number(self.value)
    }

    fn draw(&mut self, ui: &imgui::Ui) -> bool {
        ui.text(&self.node.name);
        ui.same_line();
        input_num(ui, "##constant input", &mut self.value)
    }

    fn outputs(&self) -> Option<&[OutputPin]> {
        Some(std::array::from_ref(&self.output))
    }

    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        Some(std::array::from_mut(&mut self.output))
    }
}

impl NodeSpecializationInitializer for Constant {
    fn new(node: Node) -> Self {
        let node_id = node.id();
        Self {
            node,
            value: 0.0,
            output: Pin::new_output(node_id),
        }
    }
}
