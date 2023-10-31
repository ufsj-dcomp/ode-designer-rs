use std::fs::File;
use std::io::Write;
use std::path::Path;

use std::collections::{HashMap, HashSet};

use imnodes::{InputPinId, LinkId, NodeId, OutputPinId};
use rfd::FileDialog;

use crate::core::GeneratesId;
use crate::message::{Message, MessageQueue, SendData, TaggedMessage};
use crate::nodes::{LinkEvent, Node};
use crate::pins::Pin;

use imgui::{StyleColor, StyleVar, Ui};

use crate::{nodes::NODE_SPECIALIZATIONS, pins::Sign};

#[derive(Debug, Clone)]
pub struct Link {
    pub id: LinkId,
    pub input_pin_id: InputPinId,
    pub output_pin_id: OutputPinId,
}

impl Link {
    pub fn new(input_pin_id: InputPinId, output_pin_id: OutputPinId) -> Self {
        Self {
            id: LinkId::generate(),
            input_pin_id,
            output_pin_id,
        }
    }
}

#[derive(Default)]
pub struct App {
    pub(crate) nodes: HashMap<NodeId, Box<dyn Node>>,
    pub(crate) input_pins: HashMap<InputPinId, NodeId>,
    pub(crate) output_pins: HashMap<OutputPinId, NodeId>,
    pub(crate) links: Vec<Link>,
    pub state: Option<AppState>,
    pub messages: MessageQueue,
    pub received_messages: HashMap<NodeId, HashSet<usize>>,
}

pub enum AppState {
    AddingNode { name: String, index: usize },
}

pub fn rgb(r: u8, g: u8, b: u8) -> [f32; 4] {
    [r as f32, b as f32, g as f32, 255.0].map(|x| x / 255.0)
}

pub fn input_num(ui: &Ui, label: &str, value: &mut f64) -> bool {
    let _width = ui.push_item_width(50.0);
    ui.input_scalar(label, value)
        .display_format("%.2lf")
        .build()
}

pub fn sign_pin_button(ui: &Ui, id: i32, sign: Sign) -> bool {
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
                ui.menu_bar(|| {
                    ui.menu("File", || {
                        if ui.menu_item("Save") {
                            self.save_sate();
                        }
                    })
                });

                let scope =
                    imnodes::editor(context, |mut editor| self.draw_editor(ui, &mut editor));
                if let Some(link) = scope.links_created() {
                    self.add_link(link.start_pin, link.end_pin);
                } else if let Some(link_id) = scope.get_dropped_link() {
                    self.remove_link(link_id);
                }
            });

        self.update();
    }

    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_node(&mut self, node: Box<dyn Node>) {
        let node_id = node.id();
        for input in node.inputs().unwrap_or_default() {
            self.input_pins.insert(*input.id(), node_id);
        }
        for output in node.outputs().unwrap_or_default() {
            self.output_pins.insert(*output.id(), node_id);
        }
        self.nodes.insert(node_id, node);
    }

    pub fn get_node(&self, id: NodeId) -> Option<&dyn Node> {
        self.nodes.get(&id).map(Box::as_ref)
    }

    pub fn get_link(&self, input_id: &InputPinId) -> Option<&Link> {
        self.links
            .iter()
            .find(|link| link.input_pin_id == *input_id)
    }

    pub fn remove_node(&mut self, id: &NodeId) -> Option<Box<dyn Node>> {
        let node = self.nodes.remove(id)?;
        for input in node.inputs().unwrap_or_default() {
            self.input_pins.remove(input.id());
        }
        for output in node.outputs().unwrap_or_default() {
            self.output_pins.remove(output.id());
        }
        Some(node)
    }

    fn handle_message(&mut self, tagged: TaggedMessage) -> Option<Vec<Message>> {
        match tagged.message {
            Message::SendData(SendData {
                data,
                from_output: _,
                to_input,
            }) => {
                let node_id = self.input_pins.get_mut(&to_input).unwrap();
                let node = self.nodes.get_mut(node_id).unwrap();
                let received_msgs = self.received_messages.entry(*node_id).or_default();
                if received_msgs.contains(&tagged.tag) {
                    return None;
                }
                received_msgs.insert(tagged.tag);
                node.notify(LinkEvent::Push {
                    from_pin_id: to_input,
                    payload: data.clone(),
                })
            }
            Message::AddLink(link) => {
                if self.get_link(&link.input_pin_id).is_some() {
                    return None;
                }
                try {
                    let Link {
                        ref input_pin_id,
                        ref output_pin_id,
                        ..
                    } = &link;
                    let node_ids = [
                        self.input_pins.get(input_pin_id)?,
                        self.output_pins.get(output_pin_id)?,
                    ];
                    let [input_node, output_node] = self.nodes.get_many_mut(node_ids)?;
                    if !input_node.should_link(input_pin_id) {
                        // Poor man's early return
                        None?
                    }
                    input_node
                        .get_input_mut(input_pin_id)?
                        .link_to(output_pin_id);
                    output_node
                        .get_output_mut(output_pin_id)?
                        .link_to(input_pin_id);
                    self.links.push(link);
                    output_node.broadcast_data()
                }
            }
            Message::RemoveLink(link) => {
                let Link {
                    ref input_pin_id,
                    ref output_pin_id,
                    ..
                } = &link;
                let node_ids = [
                    self.input_pins.get(input_pin_id)?,
                    self.output_pins.get(output_pin_id)?,
                ];
                let [input_node, output_node] = self.nodes.get_many_mut(node_ids)?;
                input_node
                    .get_input_mut(input_pin_id)?
                    .unlink(output_pin_id);
                output_node
                    .get_output_mut(output_pin_id)?
                    .unlink(input_pin_id);
                input_node.notify(LinkEvent::Pop(*input_pin_id))
            }
        }
    }

    pub fn add_link(&mut self, start_pin: OutputPinId, end_pin: InputPinId) {
        self.messages
            .push(Message::AddLink(Link::new(end_pin, start_pin)));
    }

    pub fn remove_link(&mut self, link_id: LinkId) {
        let Some(index) = self.links.iter().position(|link| link.id == link_id) else {
            return;
        };
        let link = self.links.swap_remove(index);
        self.messages.push(Message::RemoveLink(link));
    }

    pub fn update(&mut self) {
        let mut new_messages = MessageQueue::with_tag(self.messages.current_tag());
        for tagged in std::mem::take(&mut self.messages) {
            let tag = tagged.tag;
            let newmsgs = self.handle_message(tagged);
            for newmsg in newmsgs.unwrap_or_default() {
                new_messages.push_tagged(newmsg, tag);
            }
        }
        self.messages = new_messages;
    }

    pub fn save_sate(&self) -> Option<()> {
        let mut arguments = Vec::new();
        let mut equations = odeir::Map::new();

        for node in self.nodes.values() {
            let arg = node.to_equation_argument(self);
            arguments.push(arg);
        }

        let json = odeir::Json {
            metadata: odeir::Metadata {
                name: "TODO".to_string(),
                model_metadata: odeir::ModelMetadata::ODE(odeir::models::ode::Metadata {
                    start_time: 0.0,
                    delta_time: 0.0,
                    end_time: 0.0,
                }),
                positions: odeir::Map::new(),
            },
            arguments,
            equations,
        };

        let json_contents = serde_json::to_string_pretty(&json).ok()?;

        let file_path = FileDialog::new()
            .add_filter("json", &["json"])
            .save_file()?;

        std::fs::write(file_path, json_contents).ok()
    }
}
