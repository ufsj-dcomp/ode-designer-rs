use std::{borrow::Cow, default, io::Empty};

use imgui::Ui;

use crate::{
    message::Message,
    nodes::{Expression, Node, NodeImpl, NodeTypeRepresentation, NodeVariant},
};

use super::App;

#[derive(Debug, Default, Clone)]
pub struct SideBarState {
    node_name: String,
}
impl SideBarState {
    pub fn draw(&mut self, ui: &Ui, node_types: &[NodeTypeRepresentation]) -> Option<Node> {
        let table_group = ui.begin_group();
        let mut selected_node_type = None;

        const WIDTH: f32 = 13.0 * 7.0;

        ui.set_next_item_width(WIDTH);

        ui.input_text("##Node name", &mut self.node_name)
            .hint("Node name")
            .build();

        for node_type in node_types {
            if ui.button_with_size(&node_type.name, [WIDTH, 0.0]) {
                selected_node_type = Some(node_type)
            }
        }

        table_group.end();

        ui.same_line();
        selected_node_type.map(|nt| {
            let name = std::mem::take(&mut self.node_name);
            Node::build_from_ui(name, nt)
        })
    }
}
