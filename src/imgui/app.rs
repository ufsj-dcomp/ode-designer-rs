use std::path::Path;

use imgui::{StyleColor, StyleVar, Ui};

use crate::{
    app::{App, AppState},
    nodes::specialization::NODE_SPECIALIZATIONS,
    pins::Sign,
};

pub fn rgb(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32, b as f32, g as f32, 255.0].map(|x| x / 255.0)
}

pub fn input_num(ui: &Ui, label: &str, value: &mut f64) -> bool {
    let _width = ui.push_item_width(50.0);
    ui.input_scalar(label, value)
        .display_format("%.2lf")
        .build()
}

pub fn sign_pin_button(ui: &Ui, id: i32, sign: &Sign) -> bool {
    let (txt, col) = match sign {
        Sign::Positive => ("+", rgb(40, 200, 40)),
        Sign::Negative => ("-", rgb(200, 50, 50)),
    };
    let hover_col = col.map(|x| x * 1.25);
    let pressed_col = col.map(|x| x.powf(2.2));
    let _c = ui.push_style_color(StyleColor::Button, col);
    let _fc = ui.push_style_color(StyleColor::ButtonHovered, hover_col);
    let _hc = ui.push_style_color(StyleColor::ButtonActive, pressed_col);
    ui.button(format!("  {}  ##{}", txt, id))
}

enum StateAction {
    Keep,
    Clear,
}

impl AppState {
    fn draw(&mut self, ui: &Ui, app: &mut App) -> StateAction {
        match self {
            AppState::AddingNode { name, index } => {
                let _token = ui.push_style_var(StyleVar::PopupRounding(4.0));
                let _token = ui.push_style_var(StyleVar::WindowPadding([10.0; 2]));
                if let Some(_popup) = ui.begin_popup("Create Node") {
                    ui.text("Name");
                    ui.same_line();
                    ui.input_text("##Name", name).build();
                    ui.text("Node type");
                    ui.same_line();

                    ui.combo(
                        "##Node Type",
                        index,
                        NODE_SPECIALIZATIONS.static_slice(),
                        |names_and_specs| names_and_specs.0.into(),
                    );

                    let _token = ui.push_style_var(StyleVar::FramePadding([4.0; 2]));
                    if ui.button("Add") {
                        // let node = Node::new_with_specialization(
                        //     name.clone(),
                        //     NODE_SPECIALIZATIONS
                        //         .get(*index)
                        //         .expect(
                        //             "User tried to construct an out-of-index node specialization",
                        //         )
                        //         .1,
                        // );

                        let node_ctor = NODE_SPECIALIZATIONS
                            .get(*index)
                            .expect("User tried to construct an out-of-index node specialization")
                            .1;

                        let node = node_ctor(name.clone());

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
    pub fn save_sate(&self, _folder: impl AsRef<Path>) -> std::io::Result<()> {
        // let folder = folder.as_ref();
        // let model = self.as_model();
        /* let model = odeir::model_into_json(&model);
        let ui: &imnodes::EditorScope = todo!();
        std::fs::write(folder.join("model.json"), model) */
        Ok(())
    }
    pub fn draw_editor(&mut self, ui: &Ui, editor: &mut imnodes::EditorScope) {
        // Minimap
        editor.add_mini_map(imnodes::MiniMapLocation::BottomRight);

        // Draw nodes
        for (id, node) in self.nodes.iter_mut() {
            editor.add_node(*id, |mut ui_node| {
                if let Some(msgs) = node.process_node(ui, &mut ui_node) {
                    for msg in msgs {
                        self.messages.push(msg)
                    }
                }
            });
        }
        for link in self.links.iter() {
            editor.add_link(link.id, link.input_pin_id, link.output_pin_id);
        }
        // Enters "Create Node Popup" state
        if editor.is_hovered()
            && ui.is_mouse_clicked(imgui::MouseButton::Right)
            && self.state.is_none()
        {
            ui.open_popup("Create Node");
            self.state = Some(AppState::AddingNode {
                name: String::new(),
                index: 0,
            })
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
        ui.window("ode designer")
            .size(ui.io().display_size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .flags(flags)
            .build(|| {
                let scope =
                    imnodes::editor(context, |mut editor| self.draw_editor(ui, &mut editor));
                if let Some(link) = scope.links_created() {
                    self.add_link(&link.start_pin, &link.end_pin)
                }
            });

        self.update();
    }
}
