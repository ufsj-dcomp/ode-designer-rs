use imgui::Ui;

use crate::{nodes::{Node, NodeId, Pin, PinId}, app::App};

impl App {
    pub fn draw(&mut self, ui: &Ui, context: &mut imnodes::EditorContext, size: [f32; 2]) {
        ui.window("the best window ever")
            .size(size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .no_decoration()
            .menu_bar(true)
            .movable(false)
            .build(|| {
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
            });
    }
}
