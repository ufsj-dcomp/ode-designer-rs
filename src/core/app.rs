use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;

use imnodes::{InputPinId, LinkId, NodeId, OutputPinId};

use implot::ImVec4;
use odeir::models::ode::OdeModel;
use rfd::FileDialog;
use strum::VariantNames;

use crate::core::GeneratesId;
use crate::errors::{InvalidNodeReason, InvalidNodeReference, NotCorrectModel};
use crate::exprtree::Sign;
use crate::message::{Message, MessageQueue, SendData, TaggedMessage};
use crate::nodes::{
    LinkEvent, Node, PendingOperation, PendingOperations, NodeVariant,
};
use crate::pins::Pin;
use crate::utils::{ModelFragment, VecConversion};

use imgui::{StyleVar, Ui};

use crate::core::plot::PlotInfo;
use crate::core::plot::PlotLayout;

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

#[derive(Default)]
pub struct SimulationState {
    plots: Vec<PlotInfo>,
    plot_layout: PlotLayout,
    pub colors: Vec<ImVec4>,
    flag_simulation: bool,
    flag_plot_all: bool,
}

#[derive(Default)]
pub struct App {
    nodes: HashMap<NodeId, Node>,
    input_pins: HashMap<InputPinId, NodeId>,
    pub output_pins: HashMap<OutputPinId, NodeId>,
    links: Vec<Link>,
    state: Option<AppState>,
    queue: MessageQueue,
    received_messages: HashMap<NodeId, HashSet<usize>>,
    simulation_state: SimulationState,
}

pub enum AppState {
    AddingNode {
        name: String,
        index: usize,
        add_at_screen_space_pos: [f32; 2],
    },
    AttributingAssignerOperatesOn {
        attribute_to: NodeId,
    },
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

enum StateAction {
    Keep,
    Clear,
}

impl AppState {
    fn draw(&mut self, ui: &Ui, app: &mut App) -> StateAction {
        match self {
            AppState::AddingNode { name, index, add_at_screen_space_pos } => {
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
                        Node::VARIANTS,
                        |variant_name| (*variant_name).into(),
                    );

                    let _token = ui.push_style_var(StyleVar::FramePadding([4.0; 2]));
                    if ui.button("Add") {
                        let node_variant = NodeVariant::from_repr(*index)
                            .expect("User tried to construct an out-of-index node specialization");

                        let node_id = app.add_node(Node::build_from_ui(name.clone(), node_variant));
                        app.queue.push(
                            Message::SetNodePos {
                                node_id,
                                screen_space_pos: *add_at_screen_space_pos
                            }
                        );

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
            } => {
                ui.open_popup("Hey Modal");

                if let Some(_popup) = ui.modal_popup_config("Hey Modal")
                    .movable(false)
                    .resizable(false)
                    .scrollable(false)
                    .collapsible(false)
                    .title_bar(true)
                    .begin_popup()
                {
                    ui.text("Hey, from modal!");

                    for (node_id, node) in app.nodes.iter() {
                        if ui.selectable_config(node.name())
                            .disabled(node_id == attribute_to)
                            .build()
                        {
                            app.queue.push(
                                Message::AttributeAssignerOperatesOn {
                                    assigner_id: *attribute_to,
                                    value: *node_id,
                                }
                            );

                            return StateAction::Clear;
                        }
                    }
    
                    StateAction::Keep
                } else {
                    unreachable!("If the state is AttributingAssignerOperatesOn, then the modal is open");
                }
            },
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
                let (msgs, app_state_change) = node.process_node(ui, &mut ui_node);
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
        if editor.is_hovered()
            && ui.is_mouse_clicked(imgui::MouseButton::Right)
            && self.state.is_none()
        {
            let mouse_screen_space_pos = ui.io().mouse_pos;
            ui.open_popup("Create Node");
            self.state = Some(AppState::AddingNode {
                name: String::new(),
                index: 0,
                add_at_screen_space_pos: mouse_screen_space_pos,
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
                self.draw_menu(ui);
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

    pub fn add_node(&mut self, node: Node) -> NodeId {
        let node_id = node.id();

        for input in node.inputs().unwrap_or_default() {
            self.input_pins.insert(*input.id(), node_id);
        }

        for output in node.outputs().unwrap_or_default() {
            self.output_pins.insert(*output.id(), node_id);
        }

        let node_id_copy = node_id;
        self.nodes.insert(node_id, node);
        node_id_copy
    }

    pub fn get_node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(&id)
    }

    pub fn get_link(&self, input_id: &InputPinId) -> Option<&Link> {
        self.links
            .iter()
            .find(|link| link.input_pin_id == *input_id)
    }

    pub fn remove_node(&mut self, id: &NodeId) -> Option<Node> {
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
                    payload: data,
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
                let target_name = self.nodes.get(&value)
                    .expect("The node must have been chosen from a list of existing nodes")
                    .name()
                    .to_owned();

                let Node::Assigner(assigner) = self.nodes.get_mut(&assigner_id)
                    .expect("An assigner with this ID must exist, as it asked to open the modal")
                else {
                    panic!("This node must be an assigner. If not, how could the modal have been opened?");
                };

                assigner.operates_on = Some((value, target_name));
                None
            },
            Message::SetNodePos { node_id, screen_space_pos: [x, y] } => {
                node_id.set_position(x, y, imnodes::CoordinateSystem::ScreenSpace);
                None
            },
        }
    }

    pub fn add_link(&mut self, start_pin: OutputPinId, end_pin: InputPinId) {
        self.queue
            .push(Message::AddLink(Link::new(end_pin, start_pin, Sign::default())));
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

        self
            .nodes
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
            .for_each(|frag| {
                match frag {
                    ModelFragment::Argument(arg) => arguments.push(arg),
                    ModelFragment::Equation(eq) => equations.push(eq),
                }
            });

        odeir::Json {
            metadata: odeir::Metadata {
                name: "TODO".to_string(),
                model_metadata: odeir::ModelMetadata::ODE(odeir::models::ode::Metadata {
                    start_time: 0.0,
                    delta_time: 0.0,
                    end_time: 0.0,
                }),
                positions,
            },
            arguments,
            equations,
        }
    }

    pub fn save_state(&self) -> Option<()> {
        let file_path = FileDialog::new()
            .add_filter("json", &["json"])
            .save_file()?;

        let file = File::create(file_path).ok()?;

        let json = self.create_json();

        serde_json::to_writer_pretty(file, &json).ok()

    }

    fn try_read_model(&mut self, model: OdeModel) -> anyhow::Result<()> {
        let odeir::CoreModel { equations, arguments, positions } = model.core;

        let nodes_and_ops: Vec<(Node, Option<PendingOperations>)> = arguments
            .into_values()
            .map(Into::<ModelFragment>::into)
            .chain(
                equations
                    .into_iter()
                    .map(Into::<ModelFragment>::into)
            )
            .map(Node::build_from_fragment)
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

                        self.queue
                            .push(Message::AddLink(Link::new(via_pin_id, output_pin_id, sign)))
                    }
                    PendingOperation::SetAssignerOperatesOn { target_node_name } => {
                        let target_node = node_name_map
                            .get(&target_node_name as &str)
                            .ok_or_else(|| {
                                let source_node = self.get_node(node_id).expect("This node surely exists");
                                InvalidNodeReference {
                                    source_node: source_node.name().to_owned(),
                                    tried_linking_to: target_node_name.clone(),
                                    reason: InvalidNodeReason::NodeDoesNotExist,
                                }
                            }
                        )?;

                        self.queue
                            .push(Message::AttributeAssignerOperatesOn { assigner_id: node_id, value: target_node.id() })
                    },
                }
            }
        }

        positions
            .into_iter()
            .for_each(|(node_name, node_pos)| {
                if let Some(node) = node_name_map.get(&node_name as &str) {
                    let node_id = node.id();
                    let screen_space_pos = node_pos.convert();

                    self.queue.push(Message::SetNodePos { node_id, screen_space_pos })
                }
            });

        Ok(())
    }

    pub fn load_state(&mut self) -> anyhow::Result<()> {
        let file_path = FileDialog::new()
            .add_filter("json", &["json"])
            .pick_file()
            .ok_or_else(|| {
                std::io::Error::new(std::io::ErrorKind::NotFound, "Could not open file")
            })?;

        let file = File::open(file_path)?;

        let reader = BufReader::new(file);

        let odeir::Model::ODE(model) = serde_json::from_reader(reader)? else {
            Err(NotCorrectModel::NotODE)?
        };

        self.try_read_model(model)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use imnodes::{InputPinId, NodeId};
    

    use super::App;
    use crate::{nodes::{NodeImpl, Expression, Assigner, LinkEvent, Node, NodeVariant}, exprtree::{ExpressionNode, Operation, Sign}, core::{GeneratesId, initialize_id_generator}, pins::{OutputPin, Pin}, message::Message};

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
            self.links_to_create.push((output_pin, send_data, contribution));
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

                expr.on_link_event(link_event);
            }

            expr.into()
        }
    }

    struct AssignerNodeBuilder {
        name: String,
    }

    impl AssignerNodeBuilder {
        fn new(name: impl ToString) -> Self {
            Self { name: name.to_string() }
        }

        fn build(self, argument: &mut Node) -> Node {
            let node_id = NodeId::generate();
            let mut assigner = Assigner::new(node_id, self.name);

            let output_pin = argument
                .outputs_mut()
                .unwrap()
                .first_mut()
                .unwrap();

            output_pin.link_to(assigner.input.id);

            assigner
                .input
                .link_to(output_pin.id);

            assigner.on_link_event(
                LinkEvent::Push {
                    from_pin_id: assigner.input.id,
                    payload: argument.send_data(),
                }
            );

            assigner.into()
        }
    }

    struct AppBuilder;

    impl AppBuilder {
        fn with_nodes<const N: usize>(nodes: [Node; N]) -> App {
            let mut app = App::default();
            nodes.into_iter().for_each(|node| { app.add_node(node); });

            app
        }
    }

    const ABK_JSON: &str = include_str!("../tests/fixtures/abk.json");

    fn app_with_nodes_abk() -> App {
        let mut a = Node::build_from_ui("A".to_owned(), NodeVariant::Term);
        if let Node::Term(ref mut a) = &mut a {
            a.initial_value = 10.0;
        }

        let mut b = Node::build_from_ui("B".to_owned(), NodeVariant::Term);
        if let Node::Term(ref mut b) = &mut b {
            b.initial_value = 20.0;
        }

        let mut k = Node::build_from_ui("K".to_owned(), NodeVariant::Term);
        if let Node::Term(ref mut k) = &mut k {
            k.initial_value = 30.0;
        }

        let mut a_times_k = ExpressionNodeBuilder::new("a*k")
            .linked_to(&mut a, Sign::Positive)
            .linked_to(&mut k, Sign::Positive)
            .build(Operation::Mul);

        let mut a_times_k_plus_b = ExpressionNodeBuilder::new("a*k+b")
            .linked_to(&mut a_times_k, Sign::Positive)
            .linked_to(&mut b, Sign::Negative)
            .build(Operation::Add);

        let da_dt = AssignerNodeBuilder::new("dA/dt")
            .build(&mut a_times_k);

        let db_dt = AssignerNodeBuilder::new("dB/dt")
            .build(&mut a_times_k_plus_b);

        AppBuilder::with_nodes([
            a,
            b,
            k,
            a_times_k,
            a_times_k_plus_b,
            da_dt,
            db_dt,
        ])
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

    fn assert_expression<'app, const N: usize>(app: &'app App, name: &str, expected_connections: [(NodeId, Sign); N]) -> &'app Expression {
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

        let mut app = App::new();

        // When - The user requests to load a JSON file

        let Ok(odeir::Model::ODE(ode_model)) = serde_json::from_str(ABK_JSON) else {
            panic!("Unable to extract ODE Model from ABK JSON");
        };

        let result = app.try_read_model(ode_model);

        let expected_positions: HashMap<_, _> = [
            ("A", [-355.0, 469.0]),
            ("B", [-310.0, 175.0]),
            ("K", [0.0, 0.0]),
            ("a*k", [371.7532, 292.3206]),
            ("a*k+b", [1337.0, -240.0]),
            ("dA/dt", [357.0, 611.0]),
            ("dB/dt", [1.5, 2.5])
        ].into();

        let mut actual_positions = HashMap::new();

        // Removes node position setting messages, as they are possible via
        // raw pointer manip in Imnodes' library code, which of course depends
        // on the GUI existing
        app.queue.messages.retain(|msg| {
            if let Message::SetNodePos { node_id, screen_space_pos } = msg.message {
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
            &app, "a*k",
            [
                (a.id(), Sign::Positive),
                (k.id(), Sign::Positive),
            ]
        );

        let a_times_k_plus_b = assert_expression(
            &app, "a*k+b",
            [
                (a_times_k.id(), Sign::Positive),
                (b.id(), Sign::Negative),
            ]
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