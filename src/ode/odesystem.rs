use mexprp::{Answer, Context, Expression};
//use meval::{Context,Error,Expr};
use std::collections::BTreeMap;
use ode_solvers::*;
use std::{
    fs::File,
    io::{BufWriter, Write},
    path::Path,
};

use crate::nodes::Term;
use super::ga_json::GA_Argument;

pub type State = DVector<f64>;

#[derive(Debug, Clone, Default)]
pub struct OdeSystem {
    pub equations: BTreeMap<String, Expression<f64>>,
    pub context: Context<f64>,
}

impl OdeSystem {

    pub fn new() -> Self {
        Self {
            equations: BTreeMap::new(),
            context: Context::new(),
        }
    } 

    pub fn set_context(&mut self, args: Vec<GA_Argument>) {
        args
            .iter()
            .for_each(|arg| self.context.set_var(&arg.name, arg.value));
    }  

    pub fn update_context(&mut self, args: Vec<GA_Argument>, values: &Vec<f64>) {
        args
            .iter()
            .zip(values)
            .for_each(|(arg, value)| self.context.set_var(&arg.name, *value));
    }

    pub fn update_context_with_state(&mut self, y: &State) {
        self.equations
            .iter_mut()
            .zip(y.iter())
            .for_each(|(map, new_value)| {
                self.context.set_var(map.0, *new_value);
            });
    }
}

impl ode_solvers::System<f64, State> for OdeSystem {
    
    fn system(&mut self, _t: f64, y: &State, dydt: &mut State) {
        
        self.update_context_with_state(y);

        let mut i: usize = 0;
        for equation in self.equations.values() {
            if let Ok(Answer::Single(expr_value)) = equation.eval_ctx(&self.context) {
                dydt[i] = expr_value;
            }
            i += 1;
        }
    }
}

pub fn solve(ode_system: &OdeSystem, y: &State, t_ini: f64, t_final: f64, dt: f64) -> Vec<State> {
    
    let mut solver = Dop853::new(
        ode_system.clone(),
        t_ini,
        t_final,
        dt,
        y.clone(),
        1.0e-8,
        1.0e-8,
    );

    match solver.integrate() {
        Ok(_stats) => {
            return solver.y_out().to_vec();
        }
        Err(e) => {
            println!("An error occured: {}", e);
            return vec![];
        }
    }
}

pub fn create_ode_system(input: String, terms: impl IntoIterator<Item = Term>) -> OdeSystem {
    let mut ode_system = OdeSystem::new();
    
    for term in terms.into_iter() {
        ode_system.context.set_var(&&term.leaf.symbol.trim().to_string(), term.initial_value);
    }
    
    let lines = input.split("\n").collect::<Vec<_>>();

    for line in lines {
        let new_line = line
            .trim()
            .split('=')
            .filter(|&s| !s.is_empty())
            .collect::<Vec<_>>();

        if new_line.len() == 2 {

            let population = new_line[0].trim().to_string();
            let ode_rhs: Expression<f64> = Expression::parse(&new_line[1].trim()).unwrap();
            ode_system.equations.insert(population.clone(),ode_rhs);
        }
    }
    ode_system
}

pub fn save(times: &Vec<f64>, states: &Vec<State>, filename: &Path) {
    // Create or open file
    let file = match File::create(filename) {
        Err(e) => {
            println!("Could not open file. Error: {:?}", e);
            return;
        }
        Ok(buf) => buf,
    };
    let mut buf = BufWriter::new(file);

    // Write time and state vector in csv format
    for (i, state) in states.iter().enumerate() {
        if i >= times.len() {
            break;
        }
        buf.write_fmt(format_args!("{:.6}", times[i])).unwrap();
        for val in state.iter() {
            buf.write_fmt(format_args!(", {}", val)).unwrap();
        }
        buf.write_fmt(format_args!("\n")).unwrap();
    }
    if let Err(e) = buf.flush() {
        println!("Could not write to file. Error: {:?}", e);
    }
}
