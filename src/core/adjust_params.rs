use std::cell::{Ref, RefCell};
use std::collections::HashSet;

use imgui::{DragDropFlags, Ui};

use crate::nodes::{Node, NodeImpl, Term};

use crate::ode::ga_json::{Bound, ConfigData, GA_Argument, GA_Metadata};

pub struct Parameter {
    pub term: Term,
    pub bounds: (f32, f32),
}
#[derive(Default)]
pub struct Model {
    pub parameters: Vec<Parameter>,
    pub adjusted_parameters: RefCell<Vec<usize>>,
}

impl Model {
    pub fn new(variables: Vec<Term>) -> Self {
        let parameters = variables
            .into_iter()
            .map(|term| Parameter {
                term: term.clone(),
                bounds: (0.0, 1.0),
            })
            .collect();

        Self {
            parameters,
            adjusted_parameters: RefCell::new(Vec::new()),
        }
    }
}
