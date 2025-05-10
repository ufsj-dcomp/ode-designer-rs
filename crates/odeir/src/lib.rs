pub type Map<K, V> = std::collections::BTreeMap<K, V>;

pub mod json;
pub mod models;
pub mod transformations;

pub use json::{Json, Metadata, Model, ModelMetadata, Position};
pub use models::{Argument, Component, CoreModel, Equation};

#[cfg(test)]
mod tests {}
