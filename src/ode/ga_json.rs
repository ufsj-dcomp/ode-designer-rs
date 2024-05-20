use std::{
    fs::File,
    io::{BufReader, BufWriter, Error},
    path::Path,
};

use anyhow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct ConfigData {
    pub metadata: GA_Metadata,
    pub arguments: Vec<GA_Argument>, //manter o vetor ordenado
    pub bounds: Vec<Bound>,
}

pub fn save_json<P: AsRef<Path>>(data: ConfigData, path: P) -> anyhow::Result<(), Error> {
    let file: File = match File::create(path) {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };
    let writer: BufWriter<File> = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, &data)?;

    Ok(())
}

pub fn load_json<P: AsRef<Path>>(path: P) -> anyhow::Result<ConfigData, Error> {
    let file: File = match File::open(path) {
        Ok(f) => f,
        Err(e) => return Err(e.into()),
    };
    let reader: BufReader<File> = BufReader::new(file);
    let json: Result<ConfigData, serde_json::Error> = serde_json::from_reader(reader);

    match json {
        Ok(f) => Ok(f),
        Err(e) => return Err(e.into()),
    }
}
