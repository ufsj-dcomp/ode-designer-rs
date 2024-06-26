pub(crate) mod csvdata;
mod ga;
pub mod ga_json;
pub mod odesystem;
use crate::ode::odesystem::solve;
use ga_json::GA_Argument;
use ode_solvers::DVector;

use self::{
    csvdata::CSVData,
    ga::GA,
    ga_json::ConfigData,
    odesystem::{OdeSystem, State},
};

#[derive(Default, Debug, Clone)]
pub struct ParameterEstimation {
    ga: GA,
    pub best_solution: Vec<f64>,
    pub config_data: ConfigData,
}

//TO DO: create a thread to optimize the parameters values
//the config input file can not be changed during execution of this ga instance
impl ParameterEstimation {
    pub fn new() -> Self {
        Self {
            ga: GA::default(),
            best_solution: vec![],
            config_data: ConfigData::default(),
        }
    }

    pub fn estimate_parameters(&mut self, csv_data: CSVData, all_args: Vec<GA_Argument>, args_selected_params: Vec<GA_Argument>, mut ode_system: OdeSystem) {
  
        self.ga = GA::new(
            self.config_data.metadata.max_iterations,
            self.config_data.metadata.mutation_rate,
            self.config_data.metadata.crossover_rate,
            self.config_data.bounds.clone(),
        );

        self.ga.generate_random_population(
            self.config_data.metadata.population_size,
            self.config_data.bounds.len(),
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

        let initial_condition: State = State::from_vec(
            self.config_data
                .arguments
                .iter()
                .filter(|arg| ode_system.equations.contains_key(&arg.name))
                .map(|arg| arg.value)
                .collect(),
        );
        
        ode_system.set_context(all_args);
        
        match self.ga.optimize(|values: Vec<f64>| {
            
            //ode_system.update_context(args_selected_params.clone(), values);
            //println!("context: {:#?}", ode_system.context);

            let ode_result: Vec<DVector<f64>> = solve(
                ode_system.clone(),
                &initial_condition,
                self.config_data.metadata.start_time,
                self.config_data.metadata.end_time,
                self.config_data.metadata.delta_time,
                args_selected_params.clone(),
                values.clone()
            );

            if ode_result.len() == 0 {
                //error
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

                if (t - csv_data.time[index]).abs() < 10.0_f64.powf(-6.0) {
                    for i in 0..csv_data.labels.len() {
                        let data: f64 = csv_data.lines[i][index];
                        let dif = ode_result[ode_index][indexes[i]] - data;
                        errors[i] += dif * dif;
                    }

                    index += 1;
                }

                t += dt;
                ode_index += 1;
            }

            let sum: f64 = errors.iter().sum();
            if sum.is_nan() {
                return 1000.0;
            }

            return sum.sqrt();
        }) {
            Ok(c) => {
                println!("The best individual is {:?}", c);
                self.best_solution = c.get_values();
            }
            Err(e) => println!("An error ocurred during the optimization: {:?}", e),
        }       
    }
}
