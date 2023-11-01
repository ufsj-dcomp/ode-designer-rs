use imgui::Ui;
use imnodes::{InputPinId, NodeId};
use linkme::distributed_slice;

use crate::{
    core::app::input_num,
    core::App,
    exprtree::{ExpressionNode, Leaf, Sign},
    pins::{OutputPin, Pin},
    register_node,
};

use super::{NameAndConstructor, Node, NodeInitializer, NODE_SPECIALIZATIONS};

register_node!(Term);

#[derive(Debug)]
pub struct Term {
    id: NodeId,
    leaf: Leaf,
    initial_value: f64,
    output: OutputPin,
}

impl Node for Term {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.leaf.symbol
    }

    fn send_data(&self) -> ExpressionNode<InputPinId> {
        ExpressionNode::Leaf(self.leaf.clone())
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

impl NodeInitializer for Term {
    fn new(node_id: NodeId, name: String) -> Self {
        Self {
            id: node_id,
            leaf: Leaf {
                symbol: name,
                unary_op: Sign::Positive,
            },
            initial_value: 0.00,
            output: Pin::new(node_id),
        }
    }
}
