use linkme::distributed_slice;

use crate::{
    imgui::app::input_num,
    nodes::Data,
    pins::{OutputPin, Pin},
};

use super::{
    NameAndConstructor, NodeSpecialization, NodeSpecializationInitializer, NODE_SPECIALIZATIONS,
};

#[distributed_slice(NODE_SPECIALIZATIONS)]
static COMBINATOR_SPECIALIZATION: NameAndConstructor = ("Constant", Constant::new_boxed);

#[derive(Debug)]
pub struct Constant {
    value: f64,
    output: OutputPin,
}

impl NodeSpecialization for Constant {
    fn send_data(&self, ctx: &[super::ParentContext]) -> Data {
        Data::Number(self.value)
    }

    fn draw(&mut self, ui: &imgui::Ui, ctx: &[super::ParentContext]) -> bool {
        ui.text("TODO");
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
    fn new(node_id: imnodes::NodeId) -> Self {
        Self {
            value: 0.0,
            output: Pin::new_output(Some(node_id)),
        }
    }
}
