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

    pub fn populate_config_data(&self, selected: &RefCell<Vec<usize>>) {
        let metadata = GA_Metadata {
            name: String::from("TODO!"),
            start_time: 0.0,
            delta_time: 0.1,
            end_time: 10.0,
            population_size: 100,
            crossover_rate: 0.7,
            mutation_rate: 0.01,
            max_iterations: 1000,
        };

        let mut arguments: Vec<GA_Argument> = vec![];
        let mut bounds: Vec<Bound> = vec![];

        for id in selected.borrow().iter() {
            let parameter = &self.parameters[*id];

            arguments.push(GA_Argument::new(
                parameter.term.name().to_string(),
                parameter.term.initial_value,
            ));

            bounds.push(Bound::new(
                parameter.term.name().to_string(),
                parameter.bounds.0 as f64,
                parameter.bounds.1 as f64,
            ));
        }

        arguments.sort_by(|a, b| a.name.cmp(&b.name));

        let config_data = ConfigData {
            metadata,
            arguments,
            bounds,
        };

        println!("{:?}", config_data);
    }
}
