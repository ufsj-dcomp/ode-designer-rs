use imgui::Ui;

use crate::{nodes::{Node, NodeId, Pin, PinId}, app::App};

impl App {
    pub fn run(&mut self, ui: &Ui, context: &mut imnodes::EditorContext) {
        imnodes::editor(context, |editor| {
            for (id, node) in self.nodes.iter() {
                editor.add_node(imnodes::NodeId(*id), |ui_node| {
                    for input in node.inputs.iter() {
                        ui_node.add_input(imnodes::InputPinId(input.id), imnodes::PinShape::Triangle, || {})
                    }
                });
            }
            ;
        });
    }
}
