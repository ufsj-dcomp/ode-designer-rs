use thiserror::Error;

#[derive(Error, Debug)]
pub enum NotCorrectModel {
    #[error("Not an ODE model")]
    NotODE,

    #[error("Not a Cellular Automata model")]
    NotCellularAutomata,
}

#[derive(Error, Debug)]
#[error("{reason:?}: attempted link between `{source_node}` and `{tried_linking_to}`")]
pub struct InvalidNodeReference {
    pub source_node: String,
    pub tried_linking_to: String,
    pub reason: InvalidNodeReason,
}

#[derive(Debug, Clone, Copy)]
pub enum InvalidNodeReason {
    NodeDoesNotExist,
    NoOutputPin,
}
