
use std::{collections::HashMap, sync::atomic::AtomicI32};

use derive_more::From;

use crate::message::{Message, SendData};

pub type NodeId = i32;
pub type PinId = i32;

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
        static NEXT_ID: AtomicI32 = AtomicI32::new(0);
        Self { id: NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst), node_id: *node_id, class, linked_to: None }
    }
    pub fn new_output(node_id: &NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Output)
    }
    pub fn new_input(node_id: &NodeId) -> Self {
        Self::new_of_class(node_id, PinClass::Input)
    }
    pub fn link_to(&self, node_id: &PinId) -> Self {
        let mut this = self.clone();
        this.linked_to = Some(*node_id);
        this
    }
    pub fn unlink(&self) -> Self {
        let mut this = self.clone();
        this.linked_to = None;
        this
    }
    pub fn id(&self) -> &PinId {
        &self.node_id
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

#[derive(From, Debug, strum::EnumDiscriminants, strum::EnumVariantNames, strum::FromRepr)]
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
        static NEXT_ID: AtomicI32 = AtomicI32::new(0);
        let mut outputs = vec![];
        let inputs = vec![];
        let id = NEXT_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        match &class {
            NodeClass::Population(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Constant(_) => {
                outputs.push(Pin::new_output(&id));
            }
            NodeClass::Combinator(_) => {}
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
    pub fn receive_data(&mut self, input: &Pin, data: Data) {
        // Be sure node is indeed an input
        assert!(matches!(input.class, PinClass::Input));
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

    pub fn inputs_mut(&mut self) -> &mut [Pin] {
        &mut self.inputs
    }
    pub fn outputs_mut(&mut self) -> &mut [Pin] {
        &mut self.outputs
    }

    pub fn inputs(&self) -> &[Pin] {
        &self.inputs
    }
    pub fn outputs(&self) -> &[Pin] {
        &self.outputs
    }
}
