use std::{borrow::Cow, default, io::Empty};

use imgui::Ui;

use crate::{
    message::Message,
    nodes::{Expression, Node, NodeImpl, NodeVariant},
};

use super::App;

#[derive(Debug, Default, Clone)]
pub struct SideBarState {
    node_name: String,
}
impl SideBarState {
    pub fn draw(&mut self, ui: &Ui, node_types: &[(Cow<str>, NodeVariant)]) -> Option<Node> {
        let table_group = ui.begin_group();
        let mut node_variant = None;

        ui.set_next_item_width(13.0 * 7.0);
        
        ui.input_text("##Node name", &mut self.node_name)
            .hint("Type the node name:")
            .build();

        for (name, variant) in node_types {
            if ui.button(name) {
                node_variant = Some(variant)
            }
        }

        table_group.end();

        ui.same_line();
        node_variant.map(|variant| {
            let name = std::mem::take(&mut self.node_name);
            Node::build_from_ui(name, *variant)
        })
        
    }
}
