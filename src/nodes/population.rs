use imgui::Ui;
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

register_node!(Population);

#[derive(Debug)]
pub struct Population {
    id: NodeId,
    pub name: String,
    initial_value: f64,
    output: OutputPin,
}

impl Node for Population {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn send_data(&self) -> LinkPayload {
        LinkPayload::Text(self.name.clone())
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

    fn to_equation_argument(&self, _app: &App) -> odeir::Argument {
        odeir::Argument::Value {
            name: self.name().to_owned(),
            value: self.initial_value,
        }
    }
}

impl NodeInitializer for Population {
    fn new(node_id: NodeId, name: String) -> Self {
        Self {
            id: node_id,
            name,
            initial_value: 0.00,
            output: Pin::new(node_id),
        }
    }
}
