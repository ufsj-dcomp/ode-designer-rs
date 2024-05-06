use imgui::{DragDropFlags, Ui};

use crate::nodes::{Node, NodeImpl, Term};

pub struct Parameter {
    pub term: Term,
    pub bounds: (f32, f32),
}
#[derive(Default)]
pub struct Model {
    pub parameters: Vec<Parameter>,
    pub adjusted_parameters: Vec<Parameter>,
}

impl Model {
    pub fn new(variables: Vec<&Node>) -> Self {
        let parameters = variables
            .into_iter()
            .filter_map(|node| {
                if let Node::Term(term) = node {
                    Some(Parameter {
                        term: (*term).clone(),
                        bounds: (0.0, 1.0),
                    })
                } else {
                    None
                }
            })
            .collect();

        Self {
            parameters,
            adjusted_parameters: Vec::new(),
        }
    }
}
