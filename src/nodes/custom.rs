use std::{collections::HashMap, rc::Rc};

use imgui::ImColor32;
use imnodes::{InputPinId, NodeId};

use crate::{exprtree::{ExpressionNode, Leaf}, extensions::CustomNodeSpecification, pins::{InputPin, OutputPin, Pin}};

use super::{LinkEvent, NodeImpl, ResolutionStatus};

#[derive(Debug)]
pub struct CustomFunctionNode {
    pub id: NodeId,
    pub name: String,
    pub inputs: Vec<InputPin>,
    pub output: OutputPin,
    input_data: HashMap<InputPinId, String>,
    spec: Rc<CustomNodeSpecification>,
    formatted_args: Leaf,
}

impl CustomFunctionNode {
    pub fn from_spec(node_id: NodeId, name: String, spec: Rc<CustomNodeSpecification>) -> Self {
        Self {
            id: node_id,
            name,
            inputs: std::iter::repeat_with(|| InputPin::new(node_id))
                .take(spec.input_count())
                .collect(),
            output: Pin::new(node_id),
            input_data: HashMap::new(),
            formatted_args: Leaf {
                symbol: spec.format.format_args::<i32>([].iter()),
                unary_op: Default::default(),
            },
            spec,
        }
    }

    fn reformat_args(&mut self) {
        self.formatted_args.symbol = self.spec.format.format_args(self.input_data.values());
    }
}

impl NodeImpl for CustomFunctionNode {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn name_mut(&mut self) -> &mut String {
        &mut self.name
    }

    fn color(&self) -> imgui::ImColor32 {
        ImColor32::from_rgb(120, 0, 120)
    }

    fn selected_color(&self) -> imgui::ImColor32 {
        ImColor32::from_rgb(112, 45, 151)
    }

    fn send_data(&self) -> ExpressionNode<InputPinId> {
        ExpressionNode::Leaf(
            self.formatted_args.clone()
        )
    }

    fn on_link_event(&mut self, link_event: LinkEvent) -> bool {
        match link_event {
            LinkEvent::Push { from_pin_id, payload } => {
                self.input_data.insert(from_pin_id, payload.resolve_into_equation_part());
            }
            LinkEvent::Pop(from_pin_id) => {
                self.input_data.remove(&from_pin_id);
            }
        }

        self.reformat_args();
        true
    }

    fn draw(&mut self, ui: &imgui::Ui) -> bool {
        ui.text(&self.formatted_args.symbol);
        false
    }

    fn try_from_model_fragment(
        node_id: imnodes::NodeId,
        frag: &crate::utils::ModelFragment,
    ) -> Option<(Self, Option<super::PendingOperations>)>
    where
        Self: Sized {
        todo!()
    }

    fn to_model_fragment(&self, app: &crate::core::App) -> Option<crate::utils::ModelFragment> {
        todo!()
    }
}