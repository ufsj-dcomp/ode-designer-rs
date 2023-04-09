
use std::{collections::HashMap, sync::atomic::AtomicI32};

use derive_more::From;

pub type NodeId = i32;
pub type PinId = i32;

#[derive(Debug, Clone, Copy)]
pub enum PinClass {
    Input,
    Output,
}


#[derive(Debug, Clone, Copy)]
pub struct Pin {
    pub id: PinId,
    node_id: NodeId,
    class: PinClass,
    pub linked_to: PinId,
}

#[derive(Debug)]
pub struct Node {
    pub id: NodeId,
    pub inputs: Vec<Pin>,
    pub outputs: Vec<Pin>,
    pub class: NodeClass,
}

#[derive(Debug, Default)]
pub struct Population {
    name: String,
    initial_value: f64
}

#[derive(Debug, Default)]
pub struct Combinator {
    operation: char,
    input_exprs: HashMap<NodeId, Data>
}

#[derive(Debug, Default)]
pub struct Constant {}

#[derive(From, Debug, strum::EnumDiscriminants)]
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

#[derive(Debug, Clone)]
pub struct SendData {
    data: Data,
    from_output: PinId,
}

#[derive(Debug, Clone, From)]
pub enum Message {
    SendData(SendData)
}

impl Node {
    pub fn new_of_class(class: NodeClass) -> Self {
        static NEXT_ID: AtomicI32 = AtomicI32::new(0);
        Self {
            inputs: Vec::new(),
            outputs: Vec::new(),
            id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            class,
        }
    }
    pub fn combinator(combinator: Combinator) -> Self {
        Self::new_of_class(NodeClass::Combinator(combinator))
    }
    pub fn population(population: Population) -> Self {
        Self::new_of_class(NodeClass::Population(population))
    }
    pub fn constant(constant: Constant) -> Self {
        Self::new_of_class(NodeClass::Constant(constant))
    }
    pub fn receive_data(&mut self, input: &Pin, data: Data) {
        // Be sure node is indeed an input
        assert!(matches!(input.class, PinClass::Input));
        match &mut self.class {
            NodeClass::Combinator(combinator) => { combinator.input_exprs.insert(input.id, data); },
            NodeClass::Population(_) => todo!(),
            NodeClass::Constant(_) => unreachable!("Constant nodes don't have inputs")
        }
    }
    pub fn send_data(&self, data: Data) -> Vec<SendData> {
        self.outputs.iter().copied().map(|output| SendData { data: data.clone(), from_output: output.id }).collect()
    }
}
