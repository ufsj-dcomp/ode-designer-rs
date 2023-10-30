pub mod specialization;

use derive_more::From;
use imnodes::{InputPinId, NodeId, OutputPinId};

use crate::core::GeneratesId;

#[derive(Debug)]
pub struct Node {
    id: NodeId,
    pub name: String,
}

#[derive(Debug, Clone, From)]
pub enum LinkPayload {
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
    pub fn new(name: String) -> Self {
        Self {
            id: NodeId::generate(),
            name,
        }
    }

    pub fn id(&self) -> NodeId {
        self.id
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
