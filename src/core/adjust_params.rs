use std::collections::HashSet;

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

/*
InputFloat2
for node in self.nodes
  match node
    Node::Term(term) => all_terms.push(term.name())
   Node::Assigner(assigner) => all_populations.push(assigner.variable())
  _ => ()

all_constants = all_terms - all_pouplations 

HashSet::new()
HashSet::symmetric_difference
 */

impl Model {
    pub fn new(variables: Vec<&Node>) -> Self {
        //let all_terms: Set<String>
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
