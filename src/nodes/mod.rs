mod assigner;
pub mod expression;
pub mod term;

use std::{
    ops::{Deref, DerefMut},
    sync::LazyLock,
};

pub use expression::Expression;
pub use term::Term;
pub use assigner::Assigner;

use imgui::Ui;
use imnodes::{InputPinId, NodeId, NodeScope, OutputPinId};

use crate::{
    core::App,
    core::GeneratesId,
    exprtree::{ExpressionNode, ExpressionTree, Sign},
    message::{Message, SendData},
    pins::{InputPin, OutputPin, Pin}, utils::ModelFragment,
};

use derive_more::From;

#[derive(Debug, Clone, From)]
pub enum LinkPayload {
    Number(f64),
    Text(String),
}

pub enum LinkEvent {
    Push {
        from_pin_id: InputPinId,
        payload: ExpressionNode<InputPinId>,
    },
    Pop(InputPinId),
}

pub trait Node: std::fmt::Debug {
    fn id(&self) -> NodeId;

    fn name(&self) -> &str;

    fn on_link_event(&mut self, _link_event: LinkEvent) -> bool {
        false
    }

    fn send_data(&self) -> ExpressionNode<InputPinId>;

    fn draw(&mut self, ui: &Ui) -> bool;

    fn inputs(&self) -> Option<&[InputPin]> {
        None
    }
    fn outputs(&self) -> Option<&[OutputPin]> {
        None
    }

    fn inputs_mut(&mut self) -> Option<&mut [InputPin]> {
        None
    }
    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        None
    }

    fn broadcast_data(&self) -> Vec<Message> {
        let data = self.send_data();
        self.outputs()
            .expect("Tried broadcasting data to node without any output pins")
            .iter()
            .flat_map(|output| {
                output.linked_to.iter().copied().map(|to_input| SendData {
                    data: data.clone(),
                    from_output: output.id,
                    to_input,
                })
            })
            .map(Message::SendData)
            .collect()
    }

    fn notify(&mut self, link_event: LinkEvent) -> Option<Vec<Message>> {
        self.on_link_event(link_event)
            .then(|| self.broadcast_data())
    }

    fn state_changed(&mut self) -> bool {
        true
    }

    fn process_node(&mut self, ui: &Ui, ui_node: &mut NodeScope) -> Option<Vec<Message>> {
        ui_node.add_titlebar(|| ui.text(self.name()));

        let mut input_changed = false;

        for input in self.inputs_mut().unwrap_or_default() {
            ui_node.add_input(*input.id(), input.get_shape(), || {
                if let Some(label) = input.get_label() {
                    ui.text(label);
                }

                input_changed |= input.draw(ui);
            })
        }

        for output in self.outputs_mut().unwrap_or_default() {
            let shape = output.get_shape();
            let id = *output.id();
            ui_node.add_output(id, shape, || {});
        }

        let inner_content_changed = self.draw(ui);

        ((inner_content_changed || input_changed) && self.state_changed())
            .then(|| self.broadcast_data())
    }

    fn get_input(&self, input_pin_id: &InputPinId) -> Option<&InputPin> {
        self.inputs()
            .unwrap_or_default()
            .iter()
            .find(|pin| pin.id() == input_pin_id)
    }

    fn get_output(&self, output_pin_id: &OutputPinId) -> Option<&OutputPin> {
        self.outputs()
            .unwrap_or_default()
            .iter()
            .find(|pin| pin.id() == output_pin_id)
    }

    fn get_input_mut(&mut self, input_pin_id: &InputPinId) -> Option<&mut InputPin> {
        self.inputs_mut()
            .unwrap_or_default()
            .iter_mut()
            .find(|pin| pin.id() == input_pin_id)
    }

    fn get_output_mut(&mut self, output_pin_id: &OutputPinId) -> Option<&mut OutputPin> {
        self.outputs_mut()
            .unwrap_or_default()
            .iter_mut()
            .find(|pin| pin.id() == output_pin_id)
    }

    fn should_link(&self, input_pin_id: &InputPinId) -> bool {
        self.get_input(input_pin_id).is_some()
    }

    fn to_model_fragment(&self, app: &App) -> Option<ModelFragment>;
}

pub trait NodeInitializer {
    fn new(node_id: NodeId, name: String) -> Self;

    fn new_boxed(name: String) -> Box<dyn Node>
    where
        Self: Node + Sized + 'static,
    {
        let node_id = NodeId::generate();
        Box::new(Self::new(node_id, name))
    }

    fn try_from_model_fragment(
        node_id: NodeId,
        frag: &ModelFragment,
    ) -> Option<(Self, Option<PendingOperations>)>
    where
        Self: Node + Sized;

    fn try_from_model_fragment_boxed(
        frag: &ModelFragment,
    ) -> Option<(Box<dyn Node>, Option<PendingOperations>)>
    where
        Self: Node + Sized + 'static,
    {
        let node_id = NodeId::generate();
        

        Self::try_from_model_fragment(node_id, frag)
            .map(|(node, possible_pending_ops)| -> (Box<dyn Node>, _) {
                (Box::new(node), possible_pending_ops)
            })
    }
}

pub trait NodeFactory {
    fn new_with_name(&self, name: String) -> Box<dyn Node>;

    fn try_from_model_fragment(
        &self,
        frag: &ModelFragment,
    ) -> Option<(Box<dyn Node>, Option<PendingOperations>)>;
}

pub type NameAndFactory = (
    &'static str,
    LazyLock<&'static (dyn NodeFactory + Send + Sync)>,
);

#[linkme::distributed_slice]
pub static NODE_SPECIALIZATIONS: [NameAndFactory] = [..];

#[macro_export]
macro_rules! register_node {
    ( $node:ident ) => {
        use paste::paste;
        paste! {
            struct [<$node Factory>];

            impl $crate::nodes::NodeFactory for [<$node Factory>] {
                fn new_with_name(&self, name: String) -> Box<dyn $crate::nodes::Node> {
                    $node::new_boxed(name)
                }

                fn try_from_model_fragment(&self, frag: &$crate::utils::ModelFragment) -> Option<(Box<dyn $crate::nodes::Node>, Option<$crate::nodes::PendingOperations>)> {
                    $node::try_from_model_fragment_boxed(frag)
                }
            }

            #[linkme::distributed_slice(NODE_SPECIALIZATIONS)]
            static [<$node:upper _SPECIALIZATION>]: $crate::nodes::NameAndFactory = (stringify!($node), std::sync::LazyLock::new(|| &[<$node Factory>]));
        }
    };
}

#[derive(Debug, Default)]
pub struct ExprWrapper<T> {
    pub resolution: ResolutionStatus<Option<String>>,
    pub expr: T,
}

#[derive(Debug, Default)]
pub enum ResolutionStatus<T> {
    Resolved(T),
    #[default]
    Unresolved,
}

impl<T> ResolutionStatus<T> {
    pub fn reset(&mut self) {
        *self = ResolutionStatus::Unresolved;
    }
}

pub trait Resolvable {
    fn resolve(&self) -> String;
}

impl<T: Resolvable> Resolvable for Option<T> {
    fn resolve(&self) -> String {
        match self {
            Some(expr) => expr.resolve(),
            None => "Nothing yet!".to_string(),
        }
    }
}

impl<T: std::hash::Hash> Resolvable for ExpressionNode<T> {
    fn resolve(&self) -> String {
        self.resolve_into_equation_part()
    }
}

impl<T: std::hash::Hash> Resolvable for ExpressionTree<T> {
    fn resolve(&self) -> String {
        self.resolve_into_equation()
    }
}

impl<T: Resolvable> ExprWrapper<T> {
    pub fn get_expr_repr(&mut self) -> Option<&str> {
        if let ResolutionStatus::Unresolved = self.resolution {
            let expr = self.expr.resolve();
            self.resolution = ResolutionStatus::Resolved((!expr.is_empty()).then_some(expr));
        }

        if let ResolutionStatus::Resolved(ref expr) = self.resolution {
            expr.as_deref()
        } else {
            unreachable!("If it was not resolved, the previous `if` made sure to resolve it")
        }
    }

    pub fn set_expr(&mut self, expr: T) {
        self.resolution.reset();
        self.expr = expr;
    }
}

impl<T: Resolvable> Deref for ExprWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.expr
    }
}

impl<T: Resolvable> DerefMut for ExprWrapper<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.expr
    }
}

pub struct PendingOperations {
    pub node_id: NodeId,
    pub operations: Vec<PendingOperation>,
}

pub enum PendingOperation {
    LinkWith {
        node_name: String,
        via_pin_id: InputPinId,
        sign: Sign,
    },
}
