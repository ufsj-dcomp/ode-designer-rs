use anyhow;
use std::{
    fs::File,
    io::{BufReader, BufWriter, Error},
    path::Path,
};

#[derive(Debug, Clone, Default)]
pub struct GA_Metadata {
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
pub struct GA_Argument {
    pub name: String,
    pub value: f64,
}

impl GA_Argument {
    pub fn new(name: String, value: f64) -> Self {
        Self {
            name: name,
            value: value,
        }
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
        Self {
            name: name,
            min: min,
            max: max,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfigData {
    pub metadata: GA_Metadata,
    pub arguments: Vec<GA_Argument>, //manter o vetor ordenado
    pub bounds: Vec<Bound>,
}
