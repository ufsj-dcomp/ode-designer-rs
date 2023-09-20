use std::{collections::HashMap, fmt::Write};

use derive_more::From;
use imnodes::{InputPinId, NodeId, OutputPinId};

use crate::{
    id_gen::GeneratesId,
    message::{Message, SendData},
    pins::{InputPin, OutputPin, Pin, Sign},
};

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    pub name: String,
    pub inputs: Vec<InputPin>,
    outputs: Vec<OutputPin>,
    pub class: NodeClass,
}

#[derive(Debug, Default, Clone)]
pub struct Population {
    pub name: String,
    pub initial_value: f64,
}

#[derive(
    Debug,
    Default,
    Clone,
    Copy,
    PartialEq,
    Eq,
    strum::EnumIter,
    strum::EnumVariantNames,
    strum::FromRepr,
)]
pub enum Operation {
    #[default]
    Add,
    Sub,
    Div,
    Mul,
}

impl Operation {
    pub fn to_char(&self) -> char {
        match self {
            Self::Add => '+',
            Self::Sub => '-',
            Self::Div => '/',
            Self::Mul => '*',
        }
    }
    pub fn from_char(c: char) -> Option<Self> {
        match c {
            '+' => Some(Self::Add),
            '-' => Some(Self::Sub),
            '/' => Some(Self::Div),
            '*' => Some(Self::Mul),
            _ => None,
        }
    }
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_char())
    }
}

#[derive(Debug, Default)]
pub struct Combinator {
    pub operation: Operation,
    pub input_exprs: HashMap<InputPinId, Data>,
}

impl Combinator {
    pub fn expression_string(&self, slice: &[InputPin]) -> String {
        let search_pin = |pin_id| slice.iter().find(|pin| pin.id() == pin_id);
        self.input_exprs
            .iter()
            .map(|(key, value)| {
                let input_pin = search_pin(key).unwrap();
                match input_pin.map_data(value.clone()) {
                    Data::Number(num) => num.to_string(),
                    Data::Text(name) => name.to_string(),
                }
            })
            .collect::<Vec<_>>()
            .join(&self.operation.to_string())
    }
}

#[derive(Debug, Default)]
pub struct Constant {
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

#[derive(From, Debug, strum::EnumVariantNames, strum::EnumDiscriminants, strum::FromRepr)]
#[strum_discriminants(name(NodeClassDiscriminant))]
#[strum_discriminants(vis(pub))]
pub enum NodeClass {
    Population(Population),
    Combinator(Combinator),
    Constant(Constant),
}

#[derive(Debug, Clone, From)]
pub enum Data {
    Number(f64),
    Text(String),
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("InpuPin not found: {0:?}")]
    InputPinNotFound(InputPinId),

    #[error("Pin not found: {0:?}")]
    OutputPinNotFound(OutputPinId),
}

impl Node {
    pub fn new(name: impl Into<String>, class: NodeClass) -> Self {
        let mut outputs = vec![];
        let mut inputs = vec![];

        let id = NodeId::generate();

        match &class {
            NodeClass::Population(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Constant(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Combinator(_) => {
                outputs.push(Pin::new_output(&id));
                inputs.push(Pin::new_signed(&id, Sign::Positive));
                inputs.push(Pin::new_signed(&id, Sign::Positive));
            }
        };
        Self {
            id,
            name: name.into(),
            inputs,
            outputs,
            class,
        }
    }
    pub fn new_of_class(name: impl Into<String>, class: NodeClass) -> Self {
        Self::new(name, class)
    }
    pub fn new_combinator(name: impl Into<String>, combinator: Combinator) -> Self {
        Self::new_of_class(name, NodeClass::Combinator(combinator))
    }
    pub fn new_population(name: impl Into<String>, population: Population) -> Self {
        Self::new_of_class(name, NodeClass::Population(population))
    }
    pub fn new_constant(name: impl Into<String>, constant: Constant) -> Self {
        Self::new_of_class(name, NodeClass::Constant(constant))
    }
    pub fn receive_data(&mut self, input_pin_id: &InputPinId, data: Data) -> Vec<Message> {
        let input_pin = self.get_input(input_pin_id).expect("Pin not found");
        let data = input_pin.map_data(data);
        match &mut self.class {
            NodeClass::Combinator(combinator) => {
                combinator.input_exprs.insert(*input_pin_id, data);
            }
            NodeClass::Population(_) => todo!(),
            NodeClass::Constant(_) => unreachable!("Constant nodes don't have inputs"),
        }
        self.send_data()
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn send_data(&self) -> Vec<Message> {
        let data = match &self.class {
            NodeClass::Combinator(combinator) => {
                let mut expr = combinator.expression_string(self.inputs()).to_string();
                if !expr.is_empty() {
                    expr = format!("({})", expr)
                };
                expr.into()
            }
            NodeClass::Population(_) => Data::Text(self.name.to_string()),
            NodeClass::Constant(constant) => Data::Number(constant.value),
        };

        self.outputs
            .iter()
            .flat_map(|output| {
                output.linked_to.iter().copied().map(|to_input| SendData {
                    data: data.clone(),
                    from_output: output.id,
                    to_input,
                })
            })
            .map(Message::from)
            .collect()
    }

    pub fn should_link(&self, input_pin_id: &InputPinId) -> bool {
        self.get_input(input_pin_id).is_some()
    }

    // Boilerplate stuff

    // pub fn get_pin_mut(&mut self, pin_id: &PinId) -> Option<&mut Pin> {
    //     self.inputs
    //         .iter_mut()
    //         .find(|pin| pin.id() == pin_id)
    //         .or_else(|| self.outputs.iter_mut().find(|pin| pin.id() == pin_id))
    // }

    // pub fn get_pin(&self, pin_id: &PinId) -> Option<&Pin> {
    //     self.inputs
    //         .iter()
    //         .find(|pin| pin.id() == pin_id)
    //         .or_else(|| self.outputs.iter().find(|pin| pin.id() == pin_id))
    // }

    pub fn inputs_mut(&mut self) -> &mut [InputPin] {
        &mut self.inputs
    }
    pub fn outputs_mut(&mut self) -> &mut [OutputPin] {
        &mut self.outputs
    }

    pub fn get_input(&self, id: &InputPinId) -> Option<&InputPin> {
        self.inputs.iter().find(|pin| pin.id() == id)
    }

    pub fn get_output(&self, id: &OutputPinId) -> Option<&OutputPin> {
        self.outputs.iter().find(|pin| pin.id() == id)
    }

    pub fn get_input_mut(&mut self, id: &InputPinId) -> Option<&mut InputPin> {
        self.inputs.iter_mut().find(|pin| pin.id() == id)
    }

    pub fn get_output_mut(&mut self, id: &OutputPinId) -> Option<&mut OutputPin> {
        self.outputs.iter_mut().find(|pin| pin.id() == id)
    }

    // pub fn pins_mut(&mut self) -> impl Iterator<Item = &mut Pin> {
    //     self.inputs.iter_mut().chain(self.outputs.iter_mut())
    // }

    pub fn inputs(&self) -> &[InputPin] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[OutputPin] {
        &self.outputs
    }
}

/* impl Node {
    pub fn as_odeir_node(&self) -> Option<odeir::Node> {
        match self.class {
            NodeClass::Constant(c) => None,
            NodeClass::Combinator(c) => Some(odeir::Node::Combinator {
                id: self.id as u32,
                name: self.name.clone(),
                operation: c.operation.to_char(),
                // !TODO: make inputs
                inputs: vec![],
            }),
            NodeClass::Population(p) => Some(odeir::Node::Population {
                id: self.id as u32,
                name: self.name.clone(),
                related_constant_name: "TODO!".to_owned(),
                links: vec![],
            }),
        }
    }
    pub fn as_odeir_constant(&self) -> Option<odeir::Constant> {
        match self.class {
            NodeClass::Constant(c) => Some(odeir::Constant {
                name: self.name.clone(),
                value: c.value,
            }),
            _ => None,
        }
    }
} */
