use std::{fmt::Write};

use imnodes::{InputPinId, NodeId};
use linkme::distributed_slice;
use strum::StaticVariantsArray;

use crate::{
    core::App,
    exprtree::{ExpressionNode, ExpressionTree, Operation, Sign},
    pins::{InputPin, OutputPin, Pin},
    register_node,
};

use super::{LinkEvent, NameAndConstructor, Node, NodeInitializer, NODE_SPECIALIZATIONS};

register_node!(Expression);

#[derive(Debug)]
pub enum ResolutionStatus<T> {
    Resolved(T),
    Unresolved,
}

impl<T> ResolutionStatus<T> {
    pub fn reset(&mut self) {
        *self = ResolutionStatus::Unresolved;
    }
}

#[derive(Debug)]
pub struct Expression {
    id: NodeId,
    name: String,
    expr: ExpressionTree<InputPinId>,
    resolved_expr: ResolutionStatus<Option<String>>,
    inputs: Vec<InputPin>,
    output: OutputPin,
}

impl Expression {
    fn get_expr(&mut self) -> Option<&str> {
        if let ResolutionStatus::Unresolved = self.resolved_expr {
            let expr = self.expr.resolve_into_equation();

            self.resolved_expr = ResolutionStatus::Resolved((!expr.is_empty()).then_some(expr));
        }

        if let ResolutionStatus::Resolved(ref expr) = self.resolved_expr {
            expr.as_deref()
        } else {
            unreachable!("If it was not resolved, the previous `if` made sure to resolve it")
        }
    }
}

impl Node for Expression {
    fn id(&self) -> NodeId {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn send_data(&self) -> ExpressionNode<InputPinId> {
        ExpressionNode::SubExpr(self.expr.clone())
    }

    fn on_link_event(&mut self, link_event: LinkEvent) -> bool {
        match link_event {
            LinkEvent::Push {
                from_pin_id,
                payload,
            } => {
                let pin = self
                    .inputs
                    .iter()
                    .find(|pin| pin.id() == &from_pin_id)
                    .expect("The pin must exist if we received data through it");
                self.expr.members.insert(from_pin_id, pin.map_data(payload))
            }
            LinkEvent::Pop(from_pin_id) => self.expr.members.remove(&from_pin_id),
        };

        self.resolved_expr.reset();
        true
    }

    fn state_changed(&mut self) -> bool {
        for input in &self.inputs {
            if let Some(input_tree) = self.expr.members.get_mut(input.id()) {
                input_tree.set_unary(input.sign);
            };
        }

        self.resolved_expr.reset();
        true
    }

    fn draw(&mut self, ui: &imgui::Ui) -> bool {
        let mut selected = self.expr.join_op as usize;
        let mut changed = false;

        // Needs to be assigned to a variable other than `_`. Otherwise, the
        // style isn't applied. That's probably the case because it needs to be
        // dropped *after* the combo below has been executed.
        let _smth = ui.push_item_width(50.);

        if ui.combo(
            "##Expression operation select",
            &mut selected,
            Operation::ALL_VARIANTS,
            |op| format!("{op}").into(),
        ) {
            self.expr.join_op = Operation::from_repr(selected as u8)
                .expect("ImGui returned an out-of-range value in combobox");

            changed = true
        }

        match self.get_expr() {
            Some(expr) => ui.text(expr),
            None => ui.text("Nothing yet!"),
        };

        changed
    }

    fn inputs(&self) -> Option<&[InputPin]> {
        Some(&self.inputs)
    }

    fn outputs(&self) -> Option<&[OutputPin]> {
        Some(std::array::from_ref(&self.output))
    }

    fn inputs_mut(&mut self) -> Option<&mut [InputPin]> {
        Some(&mut self.inputs)
    }

    fn outputs_mut(&mut self) -> Option<&mut [OutputPin]> {
        Some(std::array::from_mut(&mut self.output))
    }

    fn to_equation_argument(&self, app: &App) -> odeir::Argument {
        let mut composition = Vec::with_capacity(self.inputs.len());

        for input_pin in &self.inputs {
            let Some(linked_pin_id) = input_pin.linked_to else {
                continue;
            };

            let node_id = app
                .output_pins
                .get(&linked_pin_id)
                .expect("The node must exist, otherwise this should have been unlinked");

            let node = app
                .get_node(*node_id)
                .expect("The node must exist, otherwise this should have been unlinked");

            composition.push(odeir::models::Component::Argument {
                name: node.name().to_owned(),
                contribution: input_pin.sign.into(),
            });
        }

        odeir::Argument::Composite {
            name: self.name().to_owned(),
            operation: Into::<char>::into(self.expr.join_op).into(),
            composition,
        }
    }
}

impl NodeInitializer for Expression {
    fn new(node_id: NodeId, name: String) -> Self {
        Self {
            id: node_id,
            name,
            expr: Default::default(),
            resolved_expr: ResolutionStatus::Resolved(None),
            inputs: vec![
                Pin::new_signed(node_id, Sign::Positive),
                Pin::new_signed(node_id, Sign::Positive),
            ],
            output: Pin::new(node_id),
        }
    }
}
