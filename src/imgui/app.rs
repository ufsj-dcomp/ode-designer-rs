use imgui::Ui;
use strum::VariantNames;

use crate::{nodes::{Node, PinClass, Pin, NodeClass, NodeClassDiscriminant }, app::{App, AppState}};

pub fn rgb(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32, b as f32, g as f32, 255.0]
}

impl Pin {
    fn draw(&self, ui: &Ui, ui_node: &mut imnodes::NodeScope) {
        let shape = imnodes::PinShape::Circle;
        self.id();
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
            input.draw(ui, ui_node);
        }
        for output in self.outputs().iter() {
            output.draw(ui, ui_node);
        }
        match &self.class {
            NodeClass::Constant(constant) => {
                ui.text(format!("{}: {}", self.name, constant.value));
            },
            NodeClass::Population(pop) => {
                ui.text(format!("Initial value: {}", pop.initial_value.to_string()));
            },
            NodeClass::Combinator(comb) => {
                let mut expr = comb.expression_string();
                if expr.is_empty() {
                    expr = "Nothing yet".to_string();
                }
                ui.text(expr);
            }
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
                    ui.text("Name");
                    ui.new_line();
                    ui.input_text("##Name", name).build();
                    ui.new_line();
                    ui.list_box("Node Type", index, NodeClass::VARIANTS, NodeClass::VARIANTS.len() as i32);
                    if ui.button("Add") {
                        let node = Node::new_of_class(name.clone(), NodeClass::from_repr(*index as usize).expect("Invalid index"));
                        app.add_node(node);
                        StateAction::Clear
                    } else {
                        StateAction::Keep
                    }
                } else {
                    StateAction::Clear
                
                }
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
        for link in self.links.iter() {
            editor.add_link(imnodes::LinkId(link.id), imnodes::InputPinId(link.input_pin_id), imnodes::OutputPinId(link.output_pin_id));
        }
        // Enters "Create Node Popup" state
        if editor.is_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) && self.state.is_none() {
            ui.open_popup("Create Node");
            self.state = Some(AppState::AddingNode { name: String::new(), index: 0 })
        }
        // Extra State handling
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
        let _padding = ui.push_style_var(imgui::StyleVar::WindowPadding([0.0, 0.0]));
        let _rounding = ui.push_style_var(imgui::StyleVar::WindowRounding(0.0));
        // let mut bg = ui.clone_style().colors[imgui::sys::ImGuiCol_WindowBg as usize];
        let bg = rgb(40, 40, 50);
        let _bg = ui.push_style_color(imgui::StyleColor::WindowBg, bg);
        ui.window("ode designer")
            .size(ui.io().display_size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .flags(flags)
            .build(|| {
                let scope = imnodes::editor(context, |mut editor| {
                    self.draw_editor(ui, &mut editor)
                });
                if let Some(link) = scope.links_created() {
                    self.add_link(&link.start_node.0, &link.end_pin.0);
                }
            });
    }
}
