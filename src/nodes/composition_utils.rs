use std::str::FromStr;

use imnodes::{InputPinId, NodeId};
use odeir::{models::CompositionStyle, Component};

use crate::{core::App, exprtree::{ExpressionTree, Operation}, pins::{InputPin, Pin}, utils::ModelFragment};

use super::{ExprWrapper, NodeImpl, PendingOperation, PendingOperations};

pub fn build_composition(
    name: &str,
    input_pins: &[InputPin],
    operation: String,
    style: CompositionStyle,
    app: &App,
) -> Option<ModelFragment> {
    let mut composition = Vec::with_capacity(input_pins.len());

    for input_pin in input_pins.iter() {
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

        composition.push(odeir::models::Component {
            name: node.name().to_owned(),
            contribution: input_pin.sign.into(),
        });
    }

    Some(
        odeir::Argument::Composite {
            name: name.to_owned(),
            operation,
            composition,
            style,
        }
        .into(),
    )
}

pub fn build_from_composition<N, F>(
    node_id: NodeId,
    frag: &ModelFragment,
    constructor: F,
) -> Option<(N, Option<PendingOperations>)>
where
    N: NodeImpl,
    F: FnOnce(&str, &[Component], ExprWrapper<ExpressionTree<InputPinId>>) -> N,
{
    let ModelFragment::Argument(odeir::Argument::Composite {
        name,
        operation,
        composition,
        ..
    }) = frag
    else {
        return None;
    };

    let mut expr_wrapper: ExprWrapper<ExpressionTree<InputPinId>> = Default::default();
    expr_wrapper
        .set_join_op(Operation::from_str(operation).unwrap_or_default());

    let node = constructor(name, composition, expr_wrapper);

    let pending_ops = PendingOperations {
        node_id,
        operations: composition
            .iter()
            .cloned()
            .zip(node.inputs().unwrap())
            .map(|(comp, input_pin)| {
                Some(PendingOperation::LinkWith {
                    node_name: comp.name,
                    via_pin_id: *input_pin.id(),
                    sign: comp.contribution.try_into().ok()?,
                })
            })
            .collect::<Option<Vec<PendingOperation>>>()?,
    };

    Some((node, Some(pending_ops)))
}