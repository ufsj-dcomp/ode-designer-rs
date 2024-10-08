use std::borrow::{Borrow, Cow};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::File;
use std::io::{BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use fluent_bundle::FluentValue;
use imnodes::{InputPinId, LinkId, NodeId, OutputPinId};

use implot::{ImVec4, PlotUi};
use odeir::models::ode::OdeModel;
use odeir::Argument;
use once_cell::sync::Lazy;
use rfd::FileDialog;
use strum::{VariantArray, VariantNames};
use unic_langid::LanguageIdentifier;

use crate::core::GeneratesId;
use crate::errors::{InvalidNodeReason, InvalidNodeReference, NotCorrectModel};
use crate::exprtree::Sign;
use crate::extensions::Extension;
use crate::locale::Locale;
use crate::message::{Message, MessageQueue, SendData, TaggedMessage};
use crate::nodes::{
    LinkEvent, Node, NodeImpl, NodeTypeRepresentation, NodeVariant, PendingOperation,
    PendingOperations, Term,
};
use crate::ode::csvdata;
use crate::ode::ga_json::{Bound, ConfigData, GA_Argument, GA_Metadata};
use crate::ode::odesystem::{create_ode_system, OdeSystem};
use crate::pins::Pin;
use crate::utils::{localized_error, ModelFragment, VecConversion};

use imgui::{DragDropFlags, Key, StyleVar, TabItem, TabItemFlags, Ui};

use crate::core::plot::PlotInfo;
use crate::core::plot::PlotLayout;

use super::adjust_params::{self, Parameter, ParameterEstimationState};
use super::plot::{self, CSVData};
use super::python::execute_python_code;
use super::side_bar::SideBarState;
use super::widgets;

#[derive(Debug, Clone)]
pub struct Link {
    pub id: LinkId,
    pub input_pin_id: InputPinId,
    pub output_pin_id: OutputPinId,
    pub contribution: Sign,
}

impl Link {
    pub fn new(input_pin_id: InputPinId, output_pin_id: OutputPinId, contribution: Sign) -> Self {
        Self {
            id: LinkId::generate(),
            input_pin_id,
            output_pin_id,
            contribution,
        }
    }
}

const COLORS: &[ImVec4] = &[
    ImVec4::new(0.98, 0.027, 0.027, 1.0), //vermelha
    ImVec4::new(0.09, 0.027, 0.98, 1.0),
    ImVec4::new(0.027, 0.98, 0.12, 1.0), //verde claro
    ImVec4::new(0.96, 0.98, 0.027, 1.0), //amarelo
    ImVec4::new(0.5, 0., 1.0, 1.0),      //roxo
    ImVec4::new(1.0, 0.5, 0., 1.0),      //laranja
    ImVec4::new(0.2, 1.0, 1.0, 1.0),     //ciano
    ImVec4::new(1.0, 0.2, 0.6, 1.0),     //rosa
    ImVec4::new(0.4, 0.7, 1.0, 1.0),     //azul claro
    ImVec4::new(1.0, 0.4, 0.4, 1.0),     //vermelho claro
    ImVec4::new(1.0, 0.2, 1.0, 1.0),     //magenta
    ImVec4::new(1.0, 0.7, 0.4, 1.0),     //laranja claro
    ImVec4::new(0.74, 0.055, 0.055, 1.0),
    ImVec4::new(0.6, 0.298, 0., 1.0),
    ImVec4::new(0.1, 0.4, 0.1, 1.0), //verde escuro
];

#[derive(Default, Clone)]
pub struct SimulationState {
    pub plot: PlotInfo,
    pub plot_layout: PlotLayout,
    pub colors: Vec<ImVec4>,
    pub set_focus_to_tab: bool,
    real_data: Option<csvdata::CSVData>,
}
#[derive(PartialEq)]
pub enum TabAction {
    Open,
    Close,
}
#[derive(Default)]
pub struct TextFields {
    pub x_label: String,
    pub y_label: String,
}

impl SimulationState {
    pub fn from_csv(csv_content: String, locale: &Locale) -> Self {
        let csv_data = CSVData::load_data(csv_content.as_bytes()).unwrap();

        let pane_count = csv_data.population_count().div_ceil(4);

        Self {
            plot: PlotInfo::new(csv_data, locale),
            plot_layout: PlotLayout::new(2, 2, pane_count as u32),
            colors: COLORS.to_owned(),
            set_focus_to_tab: true,
            real_data: Default::default(),
        }
    }

    pub fn draw_tabs(
        &mut self,
        ui: &Ui,
        plot_ui: &mut PlotUi,
        set_focus: bool,
        locale: &mut Locale,
    ) -> TabAction {
        let [content_width, content_height] = ui.content_region_avail();
    
        let _line_weight = implot::push_style_var_f32(&implot::StyleVar::LineWeight, 2.0);
    
        let mut opened = true;
    
        let mut flags = imgui::TabItemFlags::empty();
    
        if set_focus {
            flags.set(imgui::TabItemFlags::SET_SELECTED, true);
        }
    
        static mut ARGS: Lazy<HashMap<&'static str, FluentValue>> = Lazy::new(HashMap::new);
    
        imgui::TabItem::new(locale.get("tab-all-plots"))
            .opened(&mut opened)
            .flags(flags)
            .build(ui, || {
                implot::Plot::new(&self.plot.title)
                    .size([content_width, content_height])
                    .x_label(&self.plot.xlabel)
                    .y_label(&self.plot.ylabel)
                    .build(plot_ui, || {
                        self.plot
                            .data
                            .lines
                            .iter()
                            .zip(&self.plot.data.labels)
                            .zip(self.colors.iter().cycle())
                            .for_each(|((line, label), color)| {
                                let ImVec4 { x, y, z, w } = *color;
                                let color_token = implot::push_style_color(
                                    &implot::PlotColorElement::Line,
                                    x,
                                    y,
                                    z,
                                    w,
                                );
                                implot::PlotLine::new(label).plot(&self.plot.data.time, line);
                                color_token.pop();
                            });
    
                        if let Some(ref real_data) = self.real_data {
                            let color_token = implot::push_style_color(
                                &implot::PlotColorElement::MarkerFill,
                                1.0,
                                1.0,
                                1.0,
                                1.0,
                            );
                            real_data.lines.iter().for_each(|line| {
                                implot::PlotScatter::new("Real Data").plot(&real_data.time, line);
                            });
                            color_token.pop();
                        }
                    });
            });
    
        let populations_per_tab = (self.plot_layout.cols * self.plot_layout.rows) as usize;
        let individual_plot_size = [
            content_width / self.plot_layout.cols as f32,
            content_height / self.plot_layout.rows as f32,
        ];
    
        for (tab_idx, tab_populations) in self.plot.data.lines.chunks(populations_per_tab).enumerate() {
            // Safety: this variable is local to this function, which is not run
            // in parallel or anything of the sort (since self is mutable).
            // Therefore, it's safe to access this static variable and mutate it
            unsafe {
                ARGS.insert("idx", tab_idx.into());
            }
            imgui::TabItem::new(locale.fmt("tab-idx", unsafe { &ARGS }))
                .opened(&mut opened)
                .build(ui, || {
                    tab_populations
                        .iter()
                        .zip(&self.plot.data.labels[tab_idx * populations_per_tab..])
                        .enumerate()
                        .for_each(|(idx, (line, label))| {
                            implot::Plot::new(label)
                                .size(individual_plot_size)
                                .x_label(&self.plot.xlabel)
                                .y_label(&self.plot.ylabel)
                                .build(plot_ui, || {
                                    let ImVec4 { x, y, z, w } = self.colors
                                        [(tab_idx * populations_per_tab + idx) % self.colors.len()];
                                    let color_token = implot::push_style_color(
                                        &implot::PlotColorElement::Line,
                                        x,
                                        y,
                                        z,
                                        w,
                                    );
                                    implot::PlotLine::new(label).plot(&self.plot.data.time, line);
                                    color_token.pop();
    
                                    if let Some(ref real_data) = self.real_data {
                                        let color_token = implot::push_style_color(
                                            &implot::PlotColorElement::MarkerFill,
                                            1.0,
                                            1.0,
                                            1.0,
                                            1.0,
                                        );
                                        real_data.lines.iter().for_each(|line| {
                                            implot::PlotScatter::new("Real Data").plot(&real_data.time, line);
                                        });
                                        color_token.pop();
                                    }
                                });
    
                            if idx & 1 == 0 {
                                ui.same_line();
                            }
                        });
                });
        }
    
        if opened {
            TabAction::Open
        } else {
            TabAction::Close
        }
    }    
}

#[derive(Default)]
pub struct App {
    pub node_types: Vec<NodeTypeRepresentation>,
    nodes: HashMap<NodeId, Node>,
    input_pins: HashMap<InputPinId, NodeId>,
    pub output_pins: HashMap<OutputPinId, NodeId>,
    links: Vec<Link>,
    pub state: Option<AppState>,
    queue: MessageQueue,
    received_messages: HashMap<NodeId, HashSet<usize>>,
    pub(crate) simulation_state: Option<SimulationState>,
    pub sidebar_state: SideBarState,
    pub extensions: Vec<Extension>,
    pub text_fields: TextFields,
    pub parameter_estimation_state: Option<ParameterEstimationState>,
}

pub enum AppState {
    AddingNode {
        name: String,
        index: usize,
        add_at_screen_space_pos: [f32; 2],
    },
    AttributingAssignerOperatesOn {
        attribute_to: NodeId,
        search_query: String,
    },
    ManagingExtensions,
}

enum StateAction {
    Keep,
    Clear,
}

impl AppState {
    fn draw(&mut self, ui: &Ui, app: &mut App, locale: &Locale) -> StateAction {
        // Cancel action
        if ui.is_key_pressed(imgui::Key::Escape) {
            return StateAction::Clear;
        }

        let _token = ui.push_style_var(StyleVar::PopupRounding(4.0));
        let _token = ui.push_style_var(StyleVar::WindowPadding([10.0; 2]));

        match self {
            AppState::AddingNode {
                name,
                index,
                add_at_screen_space_pos,
            } => {
                if let Some(_popup) = ui.begin_popup(locale.get("create-node")) {
                    ui.text(locale.get("create-node-name"));
                    ui.same_line();
                    ui.input_text("##Name", name).build();
                    ui.text(locale.get("create-node-type"));
                    ui.same_line();

                    ui.combo(
                        "##Node Type",
                        index,
                        &app.node_types,
                        |NodeTypeRepresentation { name, .. }| Cow::Borrowed(name.borrow()),
                    );

                    let _token = ui.push_style_var(StyleVar::FramePadding([4.0; 2]));

                    let enter_pressed = ui.is_key_pressed(Key::Enter);

                    if ui.button(locale.get("create-node-add")) || enter_pressed {
                        let node_type = app
                            .node_types
                            .get(*index)
                            .expect("User tried to construct an out-of-index node specialization");

                        let node_id = app.add_node(Node::build_from_ui(name.clone(), node_type));
                        app.queue.push(Message::SetNodePos {
                            node_id,
                            screen_space_pos: *add_at_screen_space_pos,
                        });

                        StateAction::Clear
                    } else {
                        StateAction::Keep
                    }
                } else {
                    StateAction::Clear
                }
            }
            AppState::AttributingAssignerOperatesOn {
                attribute_to,
                ref mut search_query,
            } => {
                ui.open_popup("PopulationChooser");

                let title = locale.get("choose-pop-title");
                let title_size = ui.calc_text_size(title);
                let min_width = title_size[0];
                let min_height = title_size[1] * 12.0;

                let _win =
                    ui.push_style_var(imgui::StyleVar::WindowMinSize([min_width, min_height]));

                ui.modal_popup_config("PopulationChooser")
                    .movable(false)
                    .resizable(false)
                    .scrollable(false)
                    .collapsible(false)
                    .title_bar(false)
                    .build(|| {
                        ui.text(title);
                        widgets::search_bar(ui, search_query);
                        ui.separator();
                        ui.child_window("##population list")
                            .build(|| {
                                for (node_id, node) in app.nodes.iter().filter(|(_, node)| {
                                    node.is_assignable()
                                        && node.name().contains(search_query.as_str())
                                }) {
                                    if ui
                                        .selectable_config(node.name())
                                        .disabled(node_id == attribute_to)
                                        .build()
                                    {
                                        app.queue.push(Message::AttributeAssignerOperatesOn {
                                            assigner_id: *attribute_to,
                                            value: *node_id,
                                        });

                                        return StateAction::Clear;
                                    }
                                }
                                StateAction::Keep
                            })
                            .unwrap()
                    })
                    .expect("If the state is AttributingAssignerOperatesOn, then the modal is open")
            }
            AppState::ManagingExtensions => {
                let mut user_kept_open = true;
                ui.window(locale.get("extensions-title"))
                    .collapsible(false)
                    .opened(&mut user_kept_open)
                    .build(|| {
                        if let Some(_t) = ui.begin_table("##Extensions Table", 2) {
                            ui.table_setup_column(locale.get("extensions-origin"));
                            ui.table_setup_column(locale.get("extensions-nodes"));
                            ui.table_headers_row();

                            for ext in &app.extensions {
                                ui.table_next_row();
                                ui.table_next_column();
                                ui.text(&ext.filename);

                                ui.table_next_column();
                                for node_spec in &ext.nodes {
                                    ui.text(&node_spec.function.name);
                                }
                            }
                        }

                        if ui.button(locale.get("extensions-load")) {
                            if let Err(err) = app.pick_extension_file() {
                                eprintln!("Error opening/inspecting user extension file: {err}");
                            }
                        }
                    });

                if user_kept_open {
                    StateAction::Keep
                } else {
                    StateAction::Clear
                }
            }
        }
    }
}

impl App {
    fn is_model_valid(&self) -> bool {
        let population_ids: HashSet<_> = self.get_all_population_ids();

        let has_terms = self
            .nodes
            .iter()
            .any(|(_, node)| matches!(node, Node::Term(_)));

        let has_assigner_operating_on_term = self.nodes.iter().any(|(_, node)| {
            if let Node::Term(term) = node {
                population_ids.contains(&term.id)
            } else {
                false
            }
        });

        has_terms && has_assigner_operating_on_term
    }

    pub fn get_all_population_ids(&self) -> HashSet<&NodeId> {
        self.nodes
            .iter()
            .filter_map(|(_id, node)| match node {
                Node::Assigner(assigner) => assigner.operates_on.as_ref().map(|(id, _)| id),
                _ => None,
            })
            .collect()
    }

    pub fn get_all_populations(&self, all_population_ids: &HashSet<&NodeId>) -> Vec<Term> {
        self.nodes
            .iter()
            .filter_map(|(id, node)| match node {
                Node::Term(term) if all_population_ids.contains(id) => Some(term),
                _ => None,
            })
            .cloned()
            .collect()
    }

    pub fn get_all_constants(&self, all_population_ids: &HashSet<&NodeId>) -> Vec<Term> {
        self.nodes
            .iter()
            .filter_map(|(id, node)| match node {
                Node::Term(term) if !all_population_ids.contains(id) => Some(term),
                _ => None,
            })
            .cloned()
            .collect()
    }

    /// Draws the nodes and other elements
    pub fn draw_editor(&mut self, ui: &Ui, editor: &mut imnodes::EditorScope, locale: &Locale) {
        // Minimap
        editor.add_mini_map(imnodes::MiniMapLocation::BottomRight);

        // Draw nodes
        for (id, node) in self.nodes.iter_mut() {
            let _col = imnodes::ColorStyle::TitleBar.push_color(node.color());
            let _col2 = imnodes::ColorStyle::TitleBarSelected.push_color(node.selected_color());
            let _col3 = imnodes::ColorStyle::TitleBarHovered.push_color(node.hovered_color());
            editor.add_node(*id, |mut ui_node| {
                let (msgs, app_state_change) = node.process_node(ui, &mut ui_node, locale);
                if let Some(msgs) = msgs {
                    for msg in msgs {
                        self.queue.push(msg)
                    }
                }

                if app_state_change.is_some() {
                    self.state = app_state_change;
                }
            });
        }
        for link in self.links.iter() {
            editor.add_link(link.id, link.input_pin_id, link.output_pin_id);
        }
        // Enters "Create Node Popup" state
        if editor.is_hovered() && ui.is_mouse_clicked(imgui::MouseButton::Right) {
            let mouse_screen_space_pos = ui.io().mouse_pos;

            ui.open_popup(locale.get("create-node"));
            self.state = Some(AppState::AddingNode {
                name: String::new(),
                index: 0,
                add_at_screen_space_pos: mouse_screen_space_pos,
            })
        }
        // Extra State handling
        if let Some(mut state) = self.state.take() {
            match state.draw(ui, self, locale) {
                StateAction::Clear => self.state = None,
                StateAction::Keep => self.state = Some(state),
            }
        }
    }

    pub fn draw_main_tab(
        &mut self,
        ui: &Ui,
        context: &mut imnodes::EditorContext,
        _plot_ui: &mut PlotUi,
        locale: &Locale,
    ) {
        let _flags =
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

        let scope = imnodes::editor(context, |mut editor| {
            self.draw_editor(ui, &mut editor, locale)
        });

        if let Some(link) = scope.links_created() {
            self.add_link(link.start_pin, link.end_pin);
        } else if let Some(link_id) = scope.get_dropped_link() {
            self.remove_link(link_id);
        }

        self.update();
    }

    pub fn shortcut(&mut self, ui: &Ui) {
        if ui.is_key_down(imgui::Key::LeftCtrl) && ui.is_key_down(imgui::Key::S) {
            self.save_state();
        }

        if ui.is_key_down(imgui::Key::LeftCtrl) && ui.is_key_down(imgui::Key::N) {
            self.clear_state();
        }

        if ui.is_key_down(imgui::Key::LeftCtrl) && ui.is_key_down(imgui::Key::O) {
            self.clear_state();
            if let Err(e) = self.load_state() {
                log::error!("{e}");
            }
        }
    }

    fn plot_results(&mut self, locale: &Locale, estimated_params: Vec<(String, f64)>, real_data: Option<csvdata::CSVData>) {
        let params_str = estimated_params
            .into_iter()
            .map(|(name, value)| format!("{}={}", name, value))
            .collect::<Vec<String>>()
            .join(" ");

        let py_code = self.generate_code();

        let mut command = Command::new("python3");
        command
            .arg("-c")
            .arg(&py_code)
            .arg("--csv")
            .arg("--params")
            .arg(params_str);

        match execute_python_code(&mut command) {
            Ok(output) => {
                self.simulation_state = Some(SimulationState::from_csv(output, locale));
                if let Some(mut simulation_state) = self.simulation_state.clone() {
                    if !self.text_fields.x_label.is_empty() {
                        simulation_state.plot.xlabel = self.text_fields.x_label.to_string();
                    }
                    if !self.text_fields.y_label.is_empty() {
                        simulation_state.plot.ylabel = self.text_fields.y_label.to_string();
                    }
                    simulation_state.real_data = real_data;
                    self.simulation_state = Some(simulation_state);
                }
            }
            Err(err) => {
                localized_error!(locale, "error-python-exec");
                eprintln!("{err}");
            }
        }
    }

    pub fn draw(
        &mut self,
        ui: &Ui,
        context: &mut imnodes::EditorContext,
        plot_ui: &mut PlotUi,
        locale: &mut Locale,
    ) {
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

        ui.window("ode designer")
            .size(ui.io().display_size, imgui::Condition::Always)
            .position([0.0, 0.0], imgui::Condition::Always)
            .flags(flags)
            .build(|| {
                self.shortcut(ui);
                self.draw_menu(ui, locale);

                let tab_bar = imgui::TabBar::new("##Tabs");
                tab_bar.build(ui, || {
                    let tab_model = TabItem::new(locale.get("tab-model"));
                    tab_model.build(ui, || {
                        if let Some(node) = self.sidebar_state.draw(ui, &self.node_types, locale) {
                            self.add_node(node);
                        }
                        self.draw_main_tab(ui, context, plot_ui, locale);
                    });

                    if let Some(ref mut simulation_state) = &mut self.simulation_state {
                        let tab_action = simulation_state.draw_tabs(
                            ui,
                            plot_ui,
                            simulation_state.set_focus_to_tab,
                            locale,
                        );
                        simulation_state.set_focus_to_tab = false;

                        if tab_action == TabAction::Close {
                            self.simulation_state = None;
                        }
                    }

                    let mut opened = false;
                    if self.parameter_estimation_state.is_some() {
                        opened = true;
                    }

                    if self.is_model_valid() {
                        if ui
                            .tab_item_with_opened(
                                locale.get("tab-parameter-estimation"),
                                &mut opened,
                            )
                            .is_some()
                        {
                            if let Some(param_state) = &mut self.parameter_estimation_state {
                                param_state.draw_tables(ui, locale);
                                if ui.button(locale.get("plot-results")) {
                                    match param_state.load_real_data() {
                                        Ok(real_data) => {
                                            let estimated_params = param_state.get_estimated_parameters();
                                            self.plot_results(locale, estimated_params, Some(real_data));
                                        }
                                        Err(err) => {
                                            localized_error!(locale, "error-csv-read");
                                            eprintln!("{err}");
                                        }
                                    }
                                
                                }
                            }
                        }
                    }

                    if !opened {
                        self.parameter_estimation_state = None;
                    }

                    super::notification::render_messages(ui);
                });
            });
    }

    pub fn new(locale: &Locale) -> Self {
        Self {
            node_types: Node::VARIANTS
                .iter()
                .copied()
                .zip(NodeVariant::VARIANTS)
                .filter(|(_, variant)| variant != &&NodeVariant::Custom)
                .map(|(name, variant)| {
                    NodeTypeRepresentation::new(locale.get(name), *variant, None)
                })
                .collect(),
            ..Self::default()
        }
    }

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = node.id();

        if let Node::Term(term) = &node
            && let Some(param_state) = &mut self.parameter_estimation_state
        {
            param_state.add_variable(term.clone());
        }

        for input in node.inputs().unwrap_or_default() {
            self.input_pins.insert(*input.id(), node_id);
        }

        for output in node.outputs().unwrap_or_default() {
            self.output_pins.insert(*output.id(), node_id);
        }

        self.nodes.insert(node_id, node);
        node_id
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_link(&self, input_id: &InputPinId) -> Option<&Link> {
        self.links
            .iter()
            .find(|link| link.input_pin_id == *input_id)
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
                let mut response = node.notify(LinkEvent::Push {
                    from_pin_id: to_input,
                    payload: data,
                });
                if let Some(messages) = response.as_mut() {
                    messages.extend(node.broadcast_data());
                }
                response
            }
            Message::AddLink(link) => {
                if self.get_link(&link.input_pin_id).is_some() {
                    return None;
                }
                try {
                    let Link {
                        ref input_pin_id,
                        ref output_pin_id,
                        contribution,
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
                        .link_to((*output_pin_id, *contribution));
                    output_node
                        .get_output_mut(output_pin_id)?
                        .link_to(*input_pin_id);
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
            Message::AttributeAssignerOperatesOn { assigner_id, value } => {
                let target_name = self
                    .nodes
                    .get(&value)
                    .expect("The node must have been chosen from a list of existing nodes")
                    .name()
                    .to_owned();

                let Node::Assigner(assigner) = self
                    .nodes
                    .get_mut(&assigner_id)
                    .expect("An assigner with this ID must exist, as it asked to open the modal")
                else {
                    panic!("This node must be an assigner. If not, how could the modal have been opened?");
                };

                if let Some(param_state) = &mut self.parameter_estimation_state {
                    param_state.remove_variable(&value);
                }

                assigner
                    .operates_on
                    .replace((value, target_name))
                    .map(|(previous_node_id, _)| {
                        vec![Message::UnnatributeAssignerOperatesOn(previous_node_id)]
                    })
            }
            Message::UnnatributeAssignerOperatesOn(previous_node_id) => {
                self.parameter_estimation_state.as_ref()?;
                let term = self
                    .get_node(previous_node_id)
                    .and_then(Node::as_term)
                    .cloned();

                if let Some(param_state) = &mut self.parameter_estimation_state
                    && let Some(term) = term
                {
                    param_state.add_variable(term.clone());
                }

                None
            }
            Message::SetNodePos {
                node_id,
                screen_space_pos: [x, y],
            } => {
                node_id.set_position(x, y, imnodes::CoordinateSystem::ScreenSpace);
                None
            }
            Message::RemoveNode(node_id) => {
                let mut node = self.nodes.remove(&node_id)?;

                if let Some(param_state) = &mut self.parameter_estimation_state {
                    param_state.remove_variable(&node_id);
                }

                let mut removed_input_ids = HashSet::new();

                if let Some(input_pins) = node.inputs_mut() {
                    for input_pin in input_pins {
                        if let Some(output_pin_id) = input_pin.linked_to {
                            input_pin.unlink(&output_pin_id);
                            removed_input_ids.insert(input_pin.id);

                            let output_node_id = *self
                                .output_pins
                                .get(&output_pin_id)
                                .expect("If the pin exists, so does the node");

                            let output_node = self
                                .nodes
                                .get_mut(&output_node_id)
                                .expect("If the pin exists, so does the node");

                            output_node
                                .get_output_mut(&output_pin_id)
                                .expect("This pin surely exists")
                                .unlink(&input_pin.id);
                        }
                    }
                }

                let mut removed_output_ids = HashSet::new();
                let mut notifications = Vec::new();

                if let Some(output_pins) = node.outputs_mut() {
                    for output_pin in output_pins {
                        for input_pin_id in output_pin.linked_to.clone() {
                            output_pin.unlink(&input_pin_id);
                            removed_output_ids.insert(output_pin.id);

                            let input_node_id = *self
                                .input_pins
                                .get(&input_pin_id)
                                .expect("If the pin exists, so does the node");

                            let input_node = self
                                .nodes
                                .get_mut(&input_node_id)
                                .expect("If the pin exists, so does the node");

                            input_node
                                .get_input_mut(&input_pin_id)
                                .expect("This pin surely exists")
                                .unlink(&output_pin.id);

                            notifications.push((input_node_id, input_pin_id));
                        }
                    }
                }

                let mut links_to_remove = Vec::new();

                for link in &self.links {
                    if removed_input_ids.contains(&link.input_pin_id)
                        || removed_output_ids.contains(&link.output_pin_id)
                    {
                        links_to_remove.push(link.id);
                    }
                }

                for link_id in links_to_remove {
                    self.remove_link(link_id);
                }

                notifications
                    .into_iter()
                    .filter_map(|(input_node_id, input_pin_id)| {
                        self.nodes
                            .get_mut(&input_node_id)
                            .and_then(|input_node| input_node.notify(LinkEvent::Pop(input_pin_id)))
                    })
                    .reduce(|mut acc, notif| {
                        acc.extend(notif);
                        acc
                    })
            }
            Message::RenameNode(node_id, node_name) => {
                for (_, node) in self.nodes.iter_mut() {
                    if let Node::Assigner(asg) = node
                        && let Some((asg_node_id, _)) = asg.operates_on
                        && asg_node_id == node_id
                    {
                        asg.operates_on = Some((node_id, node_name.clone()));
                    }
                }

                if let Some(param_state) = &mut self.parameter_estimation_state {
                    param_state.rename_variable(&node_id, node_name);
                }

                None
            }
            Message::RegisterPin(node_id, pin_id) => {
                self.input_pins.insert(pin_id, node_id);
                None
            }
            Message::UnregisterPin(pin_id) => {
                self.input_pins.remove(&pin_id);
                None
            }
            Message::SetInitialValue(node_id, value) => {
                if let Some(param_state) = &mut self.parameter_estimation_state {
                    param_state.set_initial_value(&node_id, value);
                }
                None
            }
        }
    }

    pub fn add_link(&mut self, start_pin: OutputPinId, end_pin: InputPinId) {
        self.queue.push(Message::AddLink(Link::new(
            end_pin,
            start_pin,
            Sign::default(),
        )));
    }

    pub fn remove_link(&mut self, link_id: LinkId) {
        let Some(index) = self.links.iter().position(|link| link.id == link_id) else {
            return;
        };
        let link = self.links.swap_remove(index);
        self.queue.push(Message::RemoveLink(link));
    }

    pub fn update(&mut self) {
        let mut new_messages = MessageQueue::with_tag(self.queue.current_tag());
        for tagged in std::mem::take(&mut self.queue) {
            let tag = tagged.tag;
            let newmsgs = self.handle_message(tagged);
            for newmsg in newmsgs.unwrap_or_default() {
                new_messages.push_tagged(newmsg, tag);
            }
        }
        self.queue = new_messages;
    }

    fn create_json(&self) -> odeir::Json {
        let mut arguments = Vec::new();
        let mut equations = Vec::new();
        let mut positions = odeir::Map::new();

        self.nodes
            .values()
            .filter_map(|node| {
                positions.insert(
                    node.name().to_owned(),
                    #[cfg(not(test))]
                    node.id()
                        .get_position(imnodes::CoordinateSystem::ScreenSpace)
                        .convert(),
                    #[cfg(test)]
                    odeir::Position { x: 0.0, y: 0.0 },
                );

                node.to_model_fragment(self)
            })
            .for_each(|frag| match frag {
                ModelFragment::Argument(arg) => arguments.push(arg),
                ModelFragment::Equation(eq) => equations.push(eq),
            });

        odeir::Json {
            metadata: odeir::Metadata {
                name: "TODO".to_string(),
                model_metadata: odeir::ModelMetadata::ODE(self.sidebar_state.get_metadata()),
                positions,
                extension_files: self
                    .extensions
                    .iter()
                    .map(|ext| ext.filename.clone())
                    .collect(),
            },
            arguments,
            equations,
        }
    }

    pub fn generate_code(&self) -> String {
        let model: odeir::Model = self.create_json().into();

        let odeir::Model::ODE(ode_model) = model else {
            unreachable!("This program can only produce ODE models for now");
        };

        let extension_lookup_paths: Vec<_> =
            self.extensions.iter().map(|ext| &ext.file_path).collect();

        odeir::transformations::r4k::render_ode(&ode_model, &extension_lookup_paths)
    }

    pub fn generate_equations(&mut self, all_constants: Vec<Term>) {
        if self.is_model_valid() {
            let model: odeir::Model = self.create_json().into();
            let Some(param_state) = &mut self.parameter_estimation_state else {
                return;
            };

            let odeir::Model::ODE(ode_model) = model else {
                unreachable!("This program can only produce ODE models for now");
            };
            let extension_lookup_paths: Vec<_> =
                self.extensions.iter().map(|ext| &ext.file_path).collect();

            param_state.ode_system = create_ode_system(
                odeir::transformations::ode::render_txt_with_equations(
                    &ode_model,
                    &extension_lookup_paths,
                ),
                all_constants,
            );
        }
        //else Error
    }

    pub fn save_to_file(&self, content: impl AsRef<[u8]>, ext: &str) -> Option<()> {
        let file_path = FileDialog::new().add_filter(ext, &[ext]).save_file()?;

        let mut file = File::create(file_path).ok()?;
        file.write_all(content.as_ref()).ok()
    }

    pub fn save_state(&self) -> Option<()> {
        let file_path = FileDialog::new()
            .add_filter("json", &["json"])
            .save_file()?;

        let file = File::create(file_path).ok()?;

        let json = self.create_json();

        serde_json::to_writer_pretty(file, &json).ok()
    }

    fn try_read_model(&mut self, model: OdeModel, path: PathBuf) -> color_eyre::Result<()> {
        let odeir::CoreModel {
            equations,
            arguments,
            positions,
        } = model.core;

        self.sidebar_state.set_metadata(model.metadata);

        model.extension_files.into_iter().try_for_each(|file| {
            self.load_extension_from_path(
                path.parent()
                    .map(Path::to_path_buf)
                    .unwrap_or_default()
                    .join(file),
            )
        })?;

        let nodes_and_ops: Vec<(Node, Option<PendingOperations>)> = arguments
            .into_values()
            .map(Into::<ModelFragment>::into)
            .chain(equations.into_iter().map(Into::<ModelFragment>::into))
            .map(|frag| Node::build_from_fragment(frag, &self))
            .collect::<Result<_, _>>()?;

        let pending_ops: Vec<PendingOperations> = nodes_and_ops
            .into_iter()
            .filter_map(|(node, ops)| {
                self.add_node(node);
                ops
            })
            .collect();

        let mut node_name_map = HashMap::new();

        self.nodes.iter().for_each(|(_node_id, node)| {
            node_name_map.insert(node.name(), node);
        });

        for PendingOperations {
            node_id,
            operations,
        } in pending_ops
        {
            for operation in operations {
                match operation {
                    PendingOperation::LinkWith {
                        node_name,
                        via_pin_id,
                        sign,
                    } => {
                        let node_error = |reason| {
                            let source_node =
                                self.get_node(node_id).expect("This node surely exists");
                            InvalidNodeReference {
                                source_node: source_node.name().to_owned(),
                                tried_linking_to: node_name.clone(),
                                reason,
                            }
                        };

                        let output_pin_id = {
                            let node = node_name_map
                                .get(&node_name as &str)
                                .ok_or_else(|| node_error(InvalidNodeReason::NodeDoesNotExist))?;

                            let output_node = node
                                .outputs()
                                .ok_or_else(|| node_error(InvalidNodeReason::NoOutputPin))?
                                .first()
                                .ok_or_else(|| node_error(InvalidNodeReason::NoOutputPin))?;

                            *output_node.id()
                        };

                        self.queue.push(Message::AddLink(Link::new(
                            via_pin_id,
                            output_pin_id,
                            sign,
                        )))
                    }
                    PendingOperation::SetAssignerOperatesOn { target_node_name } => {
                        let target_node =
                            node_name_map
                                .get(&target_node_name as &str)
                                .ok_or_else(|| {
                                    let source_node =
                                        self.get_node(node_id).expect("This node surely exists");
                                    InvalidNodeReference {
                                        source_node: source_node.name().to_owned(),
                                        tried_linking_to: target_node_name.clone(),
                                        reason: InvalidNodeReason::NodeDoesNotExist,
                                    }
                                })?;

                        self.queue.push(Message::AttributeAssignerOperatesOn {
                            assigner_id: node_id,
                            value: target_node.id(),
                        })
                    }
                }
            }
        }

        positions.into_iter().for_each(|(node_name, node_pos)| {
            if let Some(node) = node_name_map.get(&node_name as &str) {
                let node_id = node.id();
                let screen_space_pos = node_pos.convert();

                self.queue.push(Message::SetNodePos {
                    node_id,
                    screen_space_pos,
                })
            }
        });

        Ok(())
    }

    pub fn load_state(&mut self) -> color_eyre::Result<()> {
        let file_path = FileDialog::new()
            .add_filter("json", &["json"])
            .pick_file()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Could not open file")
            })?;

        let file = File::open(&file_path)?;

        let reader = BufReader::new(file);

        let odeir::Model::ODE(model) = serde_json::from_reader(reader)? else {
            Err(NotCorrectModel::NotODE)?
        };
        self.clear_state();
        self.try_read_model(model, file_path)
    }

    pub fn clear_state(&mut self) {
        self.nodes.clear();
        self.input_pins.clear();
        self.output_pins.clear();
        self.links.clear();
        self.state = None;
        self.queue = Default::default();
        self.received_messages.clear();
        self.simulation_state = None;
        self.sidebar_state.clear_state();
        self.parameter_estimation_state.take();
    }

    pub fn update_locale(&mut self, locale: &mut Locale, lang: LanguageIdentifier) {
        locale.set_lang(lang);

        // The non-custom nodes must have their names updated. They're not
        // translated on the fly, but rather are stored in the Vec already
        // translated.
        Node::VARIANTS
            .iter()
            .zip(self.node_types.iter_mut())
            .filter(|(_, node_type)| node_type.custom_node_spec.is_none())
            .for_each(|(node_name, node_type)| {
                node_type.name = locale.get(node_name).to_owned();
            });

        if let Some(ref mut sim_state) = self.simulation_state {
            sim_state.plot.update_locale(&locale);
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        path::PathBuf,
    };

    use imnodes::{InputPinId, NodeId};

    use super::App;
    use crate::{
        core::{initialize_id_generator, GeneratesId},
        exprtree::{ExpressionNode, Operation, Sign},
        locale::Locale,
        message::Message,
        nodes::{Assigner, Expression, LinkEvent, Node, NodeImpl, SimpleNodeBuilder, Term},
        pins::{OutputPin, Pin},
    };

    struct ExpressionNodeBuilder<'pin> {
        name: String,
        links_to_create: Vec<(&'pin mut OutputPin, ExpressionNode<InputPinId>, Sign)>,
    }

    impl<'pin> ExpressionNodeBuilder<'pin> {
        fn new(name: impl ToString) -> Self {
            Self {
                name: name.to_string(),
                links_to_create: Vec::new(),
            }
        }

        fn linked_to(mut self, node: &'pin mut Node, contribution: Sign) -> Self {
            let send_data = node.send_data();
            let output_pin = node.outputs_mut().unwrap().first_mut().unwrap();
            self.links_to_create
                .push((output_pin, send_data, contribution));
            self
        }

        fn build(self, op: Operation) -> Node {
            let node_id = NodeId::generate();
            let mut expr = Expression::new(node_id, self.name);

            expr.expr_wrapper.set_join_op(op);

            let link_events: Vec<_> = expr
                .inputs_mut()
                .unwrap()
                .iter_mut()
                .zip(self.links_to_create)
                .map(|(input_pin, (output_pin, payload, sign))| {
                    let link_event = LinkEvent::Push {
                        from_pin_id: input_pin.id,
                        payload,
                    };

                    input_pin.sign = sign;

                    (input_pin.id, output_pin, sign, link_event)
                })
                .collect();

            for (input_pin_id, output_pin, sign, link_event) in link_events {
                expr.get_input_mut(&input_pin_id)
                    .unwrap()
                    .link_to((output_pin.id, sign));

                output_pin.link_to(input_pin_id);

                expr.notify(link_event);
            }

            expr.into()
        }
    }

    struct AssignerNodeBuilder {
        name: String,
    }

    impl AssignerNodeBuilder {
        fn new(name: impl ToString) -> Self {
            Self {
                name: name.to_string(),
            }
        }

        fn build(self, argument: &mut Node) -> Node {
            let node_id = NodeId::generate();
            let mut assigner = Assigner::new(node_id, self.name);

            let output_pin = argument.outputs_mut().unwrap().first_mut().unwrap();

            output_pin.link_to(assigner.input.id);

            assigner.input.link_to(output_pin.id);

            assigner.notify(LinkEvent::Push {
                from_pin_id: assigner.input.id,
                payload: argument.send_data(),
            });

            assigner.into()
        }
    }

    struct AppBuilder;

    impl AppBuilder {
        fn with_nodes<const N: usize>(nodes: [Node; N]) -> App {
            let mut app = App::default();
            nodes.into_iter().for_each(|node| {
                app.add_node(node);
            });

            app
        }
    }

    const ABK_JSON: &str = include_str!("../tests/fixtures/abk.json");

    fn app_with_nodes_abk() -> App {
        let mut a = {
            let node_id = NodeId::generate();
            let mut node = Term::new(node_id, "A".to_owned());
            node.initial_value = 10.0;
            node.into()
        };

        let mut b = {
            let node_id = NodeId::generate();
            let mut node = Term::new(node_id, "B".to_owned());
            node.initial_value = 20.0;
            node.into()
        };

        let mut k = {
            let node_id = NodeId::generate();
            let mut node = Term::new(node_id, "K".to_owned());
            node.initial_value = 30.0;
            node.into()
        };

        let mut a_times_k = ExpressionNodeBuilder::new("a*k")
            .linked_to(&mut a, Sign::Positive)
            .linked_to(&mut k, Sign::Positive)
            .build(Operation::Mul);

        let mut a_times_k_plus_b = ExpressionNodeBuilder::new("a*k+b")
            .linked_to(&mut a_times_k, Sign::Positive)
            .linked_to(&mut b, Sign::Negative)
            .build(Operation::Add);

        let da_dt = AssignerNodeBuilder::new("dA/dt").build(&mut a_times_k);

        let db_dt = AssignerNodeBuilder::new("dB/dt").build(&mut a_times_k_plus_b);

        AppBuilder::with_nodes([a, b, k, a_times_k, a_times_k_plus_b, da_dt, db_dt])
    }

    fn init_id_gen() {
        let nodesctx = imnodes::Context::new();
        let nodeseditor = nodesctx.create_editor();
        unsafe { initialize_id_generator(nodeseditor.new_identifier_generator()) };
    }

    fn assert_find_node<'app>(app: &'app App, name: &str) -> &'app Node {
        app.nodes
            .values()
            .find(|node| node.name() == name)
            .unwrap_or_else(|| panic!("Couldn't find node '{name}'"))
    }

    macro_rules! assert_get {
        ($app:expr, $node_name:expr, $node_type:tt) => {{
            let node = assert_find_node($app, $node_name);
            if let Node::$node_type(node) = node {
                node
            } else {
                panic!("Node {} is not a '{}'!", $node_name, stringify!($node_type));
            }
        }};
    }

    fn assert_expression<'app, const N: usize>(
        app: &'app App,
        name: &str,
        expected_connections: [(NodeId, Sign); N],
    ) -> &'app Expression {
        let expr = assert_get!(app, name, Expression);
        let expected_connections: HashSet<_> = expected_connections.into();

        let actual_connections: HashSet<_> = expr
            .inputs()
            .unwrap()
            .iter()
            .filter_map(|pin| {
                let Some(output_pin_id) = pin.linked_to else {
                    return None;
                };

                let linked_to_node_id = app.output_pins.get(&output_pin_id).unwrap();
                let linked_to_node = app.nodes.get(linked_to_node_id).unwrap();

                Some((linked_to_node.id(), pin.sign))
            })
            .collect();

        assert_eq!(actual_connections, expected_connections);

        expr
    }

    #[test]
    fn test_app_create_model() {
        // Given - An app with pre-populated nodes, presumably from the GUI

        init_id_gen();

        let app = app_with_nodes_abk();

        // When - The user requests a JSON to be created, for saving purposes

        let json = app.create_json();

        // Then - The JSON is expected to contain every node, correctly labeled
        // as a arguments or equations and containing their respective links

        let mut expected: odeir::Json = serde_json::from_str(ABK_JSON).unwrap();

        // Remove positions, as they can't be added in tests since they depend
        // on the GUI
        expected
            .metadata
            .positions
            .iter_mut()
            .for_each(|(_, pos)| *pos = odeir::Position { x: 0.0, y: 0.0 });

        let matching_config = assert_json_diff::Config::new(assert_json_diff::CompareMode::Strict)
            .consider_array_sorting(false);

        assert_json_diff::assert_json_matches!(json, expected, &matching_config);
    }

    #[test]
    fn test_app_read_model() {
        // Given - An empty app

        init_id_gen();

        let locale = Locale::default();
        let mut app = App::new(&locale);

        // When - The user requests to load a JSON file

        let Ok(odeir::Model::ODE(ode_model)) = serde_json::from_str(ABK_JSON) else {
            panic!("Unable to extract ODE Model from ABK JSON");
        };

        let result = app.try_read_model(ode_model, PathBuf::default());

        let expected_positions: HashMap<_, _> = [
            ("A", [-355.0, 469.0]),
            ("B", [-310.0, 175.0]),
            ("K", [0.0, 0.0]),
            ("a*k", [371.7532, 292.3206]),
            ("a*k+b", [1337.0, -240.0]),
            ("dA/dt", [357.0, 611.0]),
            ("dB/dt", [1.5, 2.5]),
        ]
        .into();

        let mut actual_positions = HashMap::new();

        // Removes node position setting messages, as they are possible via
        // raw pointer manip in Imnodes' library code, which of course depends
        // on the GUI existing
        app.queue.messages.retain(|msg| {
            if let Message::SetNodePos {
                node_id,
                screen_space_pos,
            } = msg.message
            {
                let node = app.nodes.get(&node_id).unwrap();
                actual_positions.insert(node.name(), screen_space_pos);
                false
            } else {
                true
            }
        });

        assert_eq!(actual_positions, expected_positions);

        app.update(); // Runs possible pending operations!

        // Then - The user expects all nodes to be created, with their
        // respective links

        assert!(result.is_ok());

        let a = assert_get!(&app, "A", Term);
        assert_eq!(a.initial_value, 10.0);

        let b = assert_get!(&app, "B", Term);
        assert_eq!(b.initial_value, 20.0);

        let k = assert_get!(&app, "K", Term);
        assert_eq!(k.initial_value, 30.0);

        let a_times_k = assert_expression(
            &app,
            "a*k",
            [(a.id(), Sign::Positive), (k.id(), Sign::Positive)],
        );

        let a_times_k_plus_b = assert_expression(
            &app,
            "a*k+b",
            [(a_times_k.id(), Sign::Positive), (b.id(), Sign::Negative)],
        );

        let da_dt = assert_get!(&app, "dA/dt", Assigner);
        assert!(matches!(
            da_dt.input.linked_to,
            Some(output_pin_id) if output_pin_id == a_times_k.output.id
        ));

        let db_dt = assert_get!(&app, "dB/dt", Assigner);
        assert!(matches!(
            db_dt.input.linked_to,
            Some(output_pin_id) if output_pin_id == a_times_k_plus_b.output.id
        ));
    }
}
