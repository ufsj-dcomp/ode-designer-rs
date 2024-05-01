mod ga;
mod csvdata;
pub mod ga_json;
pub mod model;

use std::{collections::BTreeMap, fs::File, io::{BufReader, Read}};
use ode_solvers::DVector;
use crate::ode::model::solve;

use self::{csvdata::CSVData, ga::GA, ga_json::{load_json, Bound, ConfigData}, model::{create_ode_system, OdeSystem, State}};
/* Objective: to find the parameter values that better adjust the set of experimental data. */

#[derive(Debug,Clone)]
pub struct ParameterEstimation {
    ga: GA,
    best_solution: Vec<f64>,
    data_file: String,
    config_data: ConfigData, 
}

//TO DO: create a thread to optimize the parameters values 
//the config input file can not be changed during execution of this ga instance
impl ParameterEstimation {

    pub fn new(file_name: String) -> Self {
        Self {
            ga: GA::default(),
            best_solution: vec![],
            data_file: file_name,
            config_data: ConfigData::default(),
        }
    }

    pub fn ode_system(&mut self, config_file_path: &str, model_file_path: &str) -> OdeSystem {
        self.config_data = match load_json(config_file_path) {
            Ok(config_model) => {println!("Config data: {:?}", config_model); config_model },
            Err(e) => {println!("Error caused by {:?}", e); ConfigData::default() },
        };
            
        let file: File = match File::open(model_file_path) {
            Ok(f) => f,
            Err(e) => {println!("Error! {:?}", e); return OdeSystem::new(self.config_data.clone());},
        };

        let input_buffer: &mut String = &mut String::from("");
        BufReader::new(file).read_to_string(input_buffer).unwrap();
    
        return create_ode_system(input_buffer.to_string(), &self.config_data);
    }
                
    pub fn estimate_parameters(&mut self, ode_system: &mut OdeSystem){
        
        match CSVData::load_data(File::open(self.data_file.clone()).unwrap()){
            Ok(csv_data) => {

                let mut bounds: BTreeMap<String,Bound> = BTreeMap::new();
                for bound in self.config_data.bounds.iter() {
                    bounds.insert(bound.name.clone(), bound.clone());
                }                

                self.ga = GA::new(
                    self.config_data.metadata.max_iterations, 
                    self.config_data.metadata.mutation_rate, 
                    self.config_data.metadata.crossover_rate, 
                    self.config_data.bounds.clone(),
                    true
                );

                self.ga.generate_random_population(
                    self.config_data.metadata.population_size, 
                    self.config_data.bounds.len()
                );                

                let mut indexes: Vec<usize> = vec![];
                for label in csv_data.labels.iter() {
                    let mut pop_index: usize = 0;
                    for key in ode_system.equations.keys() {
                        if label.trim() == key.trim() {
                            indexes.push(pop_index);
                        }
                        pop_index += 1;
                    }
                }

                let initial_condition: State = State::from_vec(ode_system.equations.keys()
                        .map(|k: &String| ode_system.get_argument_value(k.to_string())).collect());              
            
                match self.ga.optimize( |values: &Vec<f64>| {                      

                    //let mut ode_system = global_ode_system.clone();
                    ode_system.update_context(values);

                    //println!("context: {:#?}", ode_system.context);
                    
                    let ode_result: Vec<DVector<f64>> = solve(&ode_system, &initial_condition);
                    if ode_result.len() == 0 { //error 
                        return 1000.0;
                    }

                    let mut index: usize = 0;
                    let mut ode_index: usize = 0;
                    let mut t: f64 = self.config_data.metadata.start_time; 
                    let dt: f64 = self.config_data.metadata.delta_time;
                    let t_end = self.config_data.metadata.end_time;
                    let mut errors: Vec<f64> = vec![0.0; csv_data.labels.len()];   

                    while t <= t_end {
                                                
                        if index == csv_data.time.len() {
                            break;
                        }

                        if  (t - csv_data.time[index]).abs() < 10.0_f64.powf(-6.0) {

                            for i in 0..csv_data.labels.len() {                                

                                let data: f64 = csv_data.lines[i][index];
                                let dif = ode_result[ode_index][indexes[i]] - data;
                                errors[i] += dif*dif;
                            }

                            index += 1;
                        }

                        t += dt;
                        ode_index += 1;
                    }

                    let sum: f64 = errors.iter().sum();
                    if sum.is_nan(){
                        return 1000.0;
                    }
                    
                    return sum.sqrt();
                } ){                    
                    Ok(c) => { println!("The best individual is {:?}", c); self.best_solution = c.get_values(); },
                    Err(e) => println!("An error ocurred during the optimization: {:?}", e),
                }                    
            },
            Err(e) => println!("An error ocurred on reading the CSV file: {:?}", e),
        }

    }

}