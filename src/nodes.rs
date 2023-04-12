
use std::{collections::HashMap, sync::atomic::AtomicI32};

use derive_more::From;

use crate::message::{Message, SendData};

pub type NodeId = i32;
pub type PinId = i32;

pub fn next_id() -> i32 {
    NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst)
}

static NEXT_ID: AtomicI32 = AtomicI32::new(0);

#[derive(Debug, Clone)]
pub enum PinClass {
    Input,
    Output,
}


#[derive(Debug, Clone)]
pub struct Pin {
    id: PinId,
    node_id: NodeId,
    class: PinClass,
    linked_to: Option<PinId>,
}

impl Pin {
    pub fn new_of_class(node_id: &NodeId, class: PinClass) -> Self {
        Self { id: next_id(), node_id: *node_id, class, linked_to: None }
    }
    pub fn new_output(node_id: &NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Output)
    }
    pub fn new_input(node_id: &NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Input)
    }
    pub fn link_to(&mut self, pin_id: &PinId) {
        self.linked_to = Some(*pin_id);
    }
    pub fn unlink(&mut self) {
        self.linked_to = None;
    }
    pub fn id(&self) -> &PinId {
        &self.id
    }
    pub fn linked_to(&self) -> Option<&PinId> {
        self.linked_to.as_ref()
    }
    pub fn class(&self) -> &PinClass {
        &self.class
    }

}

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    pub name: String,
    inputs: Vec<Pin>,
    outputs: Vec<Pin>,
    pub class: NodeClass,
}

#[derive(Debug, Default, Clone)]
pub struct Population {
    pub name: String,
    pub initial_value: f64
}

#[derive(Debug, Default)]
pub struct Combinator {
    pub operation: char,
    pub input_exprs: HashMap<NodeId, Data>
}

impl Combinator {
    pub fn expression_string(&self) -> String {
        self.input_exprs.values().map(|value| match value {
            Data::Number(num) => num.to_string(),
            Data::Name(name) => name.to_string(),
        }).collect::<Vec<_>>().join(&self.operation.to_string())
    }
}

#[derive(Debug, Default)]
pub struct Constant {pub value: f64}

impl Constant {
    pub fn new(value: f64) -> Self { Self { value } }
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
    Name(String),
}


impl Node {
    pub fn new_of_class(name: impl Into<String>, class: NodeClass) -> Self {
        let mut outputs = vec![];
        let mut inputs = vec![];
        let id = next_id();
        match &class {
            NodeClass::Population(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Constant(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Combinator(_) => {
                outputs.push(dbg!(Pin::new_output(&id)));
                inputs.push(dbg!(Pin::new_input(&id)));
            }
        };
        Self {
            name: name.into(),
            inputs,
            outputs,
            id,
            class,
        }
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
    pub fn receive_data(&mut self, input_pin_id: &PinId, data: Data) {
        let input = self.inputs.iter().find(|pin| pin.id() == input_pin_id).expect("PinId not found");
        match &mut self.class {
            NodeClass::Combinator(combinator) => { combinator.input_exprs.insert(input.id, data); },
            NodeClass::Population(_) => todo!(),
            NodeClass::Constant(_) => unreachable!("Constant nodes don't have inputs")
        }
    }

    pub fn id(&self) -> &NodeId {
        &self.id
    }

    pub fn send_data(&self, data: Data) -> Vec<Message> {
        self.outputs.iter().filter_map(|output| Some(SendData { data: data.clone(), from_output: output.id, to_input: output.linked_to? })).map(Message::from).collect()
    }

    pub fn get_pin_mut(&mut self, pin_id: &PinId) -> Option<&mut Pin> {
        self.inputs.iter_mut().find(|pin| pin.id() == pin_id).or_else(|| self.outputs.iter_mut().find(|pin| pin.id() == pin_id))
    }

    pub fn get_pin(&self, pin_id: &PinId) -> Option<&Pin> {
        self.inputs.iter().find(|pin| pin.id() == pin_id).or_else(|| self.outputs.iter().find(|pin| pin.id() == pin_id))
    }

    pub fn inputs_mut(&mut self) -> &mut [Pin] {
        &mut self.inputs
    }
    pub fn outputs_mut(&mut self) -> &mut [Pin] {
        &mut self.outputs
    }

    pub fn get_input(&self, id: &PinId) -> Option<&Pin> {
        self.inputs.iter().find(|pin| pin.id() == id)
    }

    pub fn get_output(&self, id: &PinId) -> Option<&Pin> {
        self.outputs.iter().find(|pin| pin.id() == id)
    }

    pub fn get_input_mut(&mut self, id: &PinId) -> Option<&mut Pin> {
        self.inputs.iter_mut().find(|pin| pin.id() == id)
    }
    
    pub fn get_output_mut(&mut self, id: &PinId) -> Option<&mut Pin> {
        self.outputs.iter_mut().find(|pin| pin.id() == id)
    }

    pub fn inputs(&self) -> &[Pin] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[Pin] {
        &self.outputs
    }
}
