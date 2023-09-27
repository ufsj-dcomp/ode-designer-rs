use imgui::Ui;
use linkme::distributed_slice;

use crate::{
    imgui::app::input_num,
    nodes::Data,
    pins::{OutputPin, Pin},
};

use super::{
    NameAndConstructor, NodeSpecialization, NodeSpecializationInitializer, ParentContext,
    NODE_SPECIALIZATIONS,
};

#[distributed_slice(NODE_SPECIALIZATIONS)]
static POPULATION_SPECIALIZATION: NameAndConstructor = ("Population", Population::new_boxed);

#[derive(Debug, Clone)]
pub struct Population {
    name: String,
    initial_value: f64,
    output: OutputPin,
}

impl NodeSpecialization for Population {
    fn send_data(&self, ctx: &[ParentContext]) -> Data {
        // let Some(ParentContext::String(parent_name)) = ctx.get(0) else {
        //     panic!("Population Node Specialization didn't receive context from its parent during a call to `send_data`")
        // };
        Data::Text(self.name.to_string())
    }

    fn draw(&mut self, ui: &Ui, ctx: &[ParentContext]) -> bool {
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
    fn new(node_id: imnodes::NodeId) -> Self {
        Self {
            name: "TODO".to_string(),
            initial_value: 0.00,
            output: Pin::new_output(Some(node_id)),
        }
    }
}
