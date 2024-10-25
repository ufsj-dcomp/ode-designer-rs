#![allow(dead_code)] // A lot of stuff is WIP here

#[derive(Debug, Clone, Default)]
pub struct GAMetadata {
    pub name: String,
    pub start_time: f64,
    pub delta_time: f64,
    pub end_time: f64,
    pub population_size: usize,
    pub crossover_rate: f64,
    pub mutation_rate: f64,
    pub max_iterations: usize,
}

//initial condition
#[derive(Debug, Clone, Default)]
pub struct GAArgument {
    pub name: String,
    pub value: f64,
}

impl GAArgument {
    pub fn new(name: String, value: f64) -> Self {
        Self { name, value }
    }
}

//parameters to be adjusted
#[derive(Debug, Clone, Default)]
pub struct Bound {
    pub name: String,
    pub min: f64,
    pub max: f64,
}

impl Bound {
    pub fn new(name: String, min: f64, max: f64) -> Self {
        Self { name, min, max }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfigData {
    pub metadata: GAMetadata,
    pub arguments: Vec<GAArgument>, //manter o vetor ordenado
    pub bounds: Vec<Bound>,
}
