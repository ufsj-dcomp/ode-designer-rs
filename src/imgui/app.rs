use imgui::Ui;
use strum::VariantNames;

use crate::{nodes::{Node, PinClass, Pin, NodeClass, NodeClassDiscriminant }, app::{App, AppState}};

impl Pin {
    fn draw(&self, ui: &Ui, ui_node: &mut imnodes::NodeScope) {
        let shape = imnodes::PinShape::Circle;
        match self.class() {
            PinClass::Input => ui_node.add_input(imnodes::InputPinId(*self.id()), shape, || {}),
            PinClass::Output => ui_node.add_output(imnodes::OutputPinId(*self.id()), shape, || {}),
        }
    }

}

impl Node {
    fn draw(&self, ui: &Ui, ui_node: &mut imnodes::NodeScope) {
        ui_node.add_titlebar(|| ui.text(&self.name));
        for input in self.inputs().iter() {
            input.draw(ui, ui_node)
        }
        for output in self.outputs().iter() {
            output.draw(ui, ui_node)
        }
    }
}

enum StateAction {
    Keep,
    Clear
}

impl AppState {
    fn draw(&mut self, ui: &Ui, app: &mut App) -> StateAction {
        match self {
            AppState::AddingNode { name, index, .. } => {
                if let Some(_popup) = ui.begin_popup("Create Node") {
                    ui.input_text("Name", name).build();
                    ui.list_box("Node Type", index, NodeClass::VARIANTS, 0);
                    if ui.button("Add") {
                        let node = Node::new_of_class(name.clone(), NodeClass::from_repr(*index as usize).expect("Invalid index"));
                        app.add_node(node);
                        StateAction::Clear
                    } else {
                        StateAction::Keep
                    }
                } else { unreachable!() }
            }
        }
    }

}

impl App {
    pub fn draw_editor(&mut self, ui: &Ui, editor: &mut imnodes::EditorScope) {
        for (id, node) in self.nodes.iter() {
            editor.add_node(imnodes::NodeId(*id), |mut ui_node| {
                node.draw(ui, &mut ui_node);
            });
        }
        if editor.is_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) && self.state.is_none() {
            ui.open_popup("Create Node");
            self.state = Some(AppState::AddingNode { name: String::new(), index: 0 })
        }
        if let Some(mut state) = self.state.take() {
            match state.draw(ui, self) {
                StateAction::Clear => self.state = None,
                StateAction::Keep => self.state = Some(state),
            }
        }
    }
    pub fn draw(&mut self, ui: &Ui, context: &mut imnodes::EditorContext) {
          let flags =
        // No borders etc for top-level window
        imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_MOVE
        // Show menu bar
        | imgui::WindowFlags::MENU_BAR
        // Don't raise window on focus (as it'll clobber floating windows)
        | imgui::WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS | imgui::WindowFlags::NO_NAV_FOCUS
        // Don't want the dock area's parent to be dockable!
        | imgui::WindowFlags::NO_DOCKING
        ;

    // Remove padding/rounding on main container window
        let _mw_style_tweaks = {
            let padding = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
            let rounding = ui.push_style_var(imgui::StyleVar::WindowRounding(0.0));
            (padding, rounding)
        };
        ui.window("the best window ever")
            .size(ui.io().display_size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .flags(flags)
            .build(|| {
                imnodes::editor(context, |mut editor| {
                    self.draw_editor(ui, &mut editor)
                });
            });
    }
}
