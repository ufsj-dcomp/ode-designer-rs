use imnodes::NodeId;
use linkme::distributed_slice;

use crate::{
    core::app::input_num,
    core::App,
    nodes::LinkPayload,
    pins::{OutputPin, Pin},
    register_node,
};

use super::{NameAndConstructor, Node, NodeInitializer, NODE_SPECIALIZATIONS};

register_node!(Constant);

#[derive(Debug)]
pub struct Constant {
    id: NodeId,
    pub name: String,
    value: f64,
    output: OutputPin,
}

impl Node for Constant {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn send_data(&self) -> LinkPayload {
        LinkPayload::Number(self.value)
    }

    fn draw(&mut self, ui: &imgui::Ui) -> bool {
        ui.text(&self.name);
        ui.same_line();
        input_num(ui, "##constant input", &mut self.value)
    }

    fn outputs(&self) -> Option<&[OutputPin]> {
        Some(std::array::from_ref(&self.output))
    }

    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        Some(std::array::from_mut(&mut self.output))
    }

    fn to_equation_argument(&self, _app: &App) -> odeir::Argument {
        odeir::Argument::Value {
            name: self.name().to_owned(),
            value: self.value,
        }
    }
}

impl NodeInitializer for Constant {
    fn new(node_id: NodeId, name: String) -> Self {
        Self {
            id: node_id,
            name,
            value: 0.0,
            output: Pin::new(node_id),
        }
    }
}
