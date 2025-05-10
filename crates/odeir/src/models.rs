
use serde::{Deserialize, Serialize};

use crate::{Map, Position};

pub mod cellular_automata;
pub mod ode;

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct CoreModel {
    pub arguments: Map<String, Argument>,
    pub equations: Vec<Equation>,
    pub positions: Map<String, Position>,
}

impl CoreModel {
    pub fn new() -> Self {
        Default::default()
    }
    pub fn insert_argument(&mut self, arg: Argument) {
        self.arguments.insert(arg.name().to_owned(), arg);
    }
    pub fn insert_equation(&mut self, eq: Equation) {
        self.equations.push(eq);
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum Argument {
    Value {
        name: String,
        value: f64,
    },
    Composite {
        name: String,
        operation: String,
        #[serde(default)]
        style: CompositionStyle,
        composition: Vec<Component>,
    },
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
pub enum CompositionStyle {
    #[default]
    Infixed,
    Prefixed,
}

impl Argument {
    pub fn name(&self) -> &str {
        match self {
            Argument::Value { name, .. } => name,
            Argument::Composite { name, .. } => name,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Component {
    pub name: String,
    pub contribution: char,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Equation {
    pub name: String,
    pub operates_on: Option<String>,
    pub argument: String,
    pub contribution: char,
}

