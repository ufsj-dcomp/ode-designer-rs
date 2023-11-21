use thiserror::Error;

use crate::utils::ModelFragment;

#[derive(Debug, Error)]
#[error("Could not build a node from this fragment: {0:#?}")]
pub struct NotANode(pub ModelFragment);
