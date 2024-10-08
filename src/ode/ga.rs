use anyhow::Error;
use quicksort::quicksort_by;
use rand::Rng;
use rayon::iter::{IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelIterator};
use std::cmp::Ordering;
use std::fmt;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::vec;
use vecshard::ShardExt;

use super::ga_json::Bound;

#[derive(Debug, Clone, Default)]
pub struct Chromosome {
    values: Vec<f64>,
    pub fitness: f64,
}

const MUTATION_PERCENTAGE: f64 = 0.1;

impl Chromosome {
    pub fn new_empty() -> Self {
        Self {
            values: vec![],
            fitness: 0.0,
        }
    }

    pub fn new(v: Vec<f64>) -> Self {
        Self {
            values: v,
            fitness: 0.0,
        }
    }

    pub fn get_values(&self) -> Vec<f64> {
        return self.values.clone();
    }

    fn mutation(&mut self, mutation_rate: f64, bounds: &Vec<Bound>) {
        let mut rng = rand::thread_rng();
        let c_index: usize = rng.gen_range(0..self.values.len());
        let mut p: f64 = rng.gen_range(0.0..=1.0);

        if p < mutation_rate {
            p = rng.gen_range(0.0..=1.0);

            let value = MUTATION_PERCENTAGE * self.values[c_index];

            if p < 0.5 {
                self.values[c_index] += value;
                if self.values[c_index] > bounds[c_index].max {
                    self.values[c_index] = bounds[c_index].max;
                }
            } else {
                self.values[c_index] -= value;
                if self.values[c_index] < bounds[c_index].min {
                    self.values[c_index] = bounds[c_index].min;
                }
            }
        }
    }
}

impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[fitness = {}, values = [", self.fitness)?;

        for v in self.values.iter() {
            write!(f, "{}, ", v)?;
        }

        write!(f, "] ]\n")
    }
}

#[derive(Debug, Clone, Default)]
pub struct GA {
    max_generations: usize,
    mutation_rate: f64,
    crossover_rate: f64,
    pub population: Vec<Chromosome>,
    bounds: Vec<Bound>, //bound for each chromosome
}

impl GA {
    pub fn new(max: usize, mut_rate: f64, cross_rate: f64, bounds: Vec<Bound>) -> Self {
        Self {
            max_generations: max,
            mutation_rate: mut_rate,
            crossover_rate: cross_rate,
            population: vec![],
            bounds: bounds,
        }
    }

    pub fn generate_random_population(&mut self, p_size: usize, c_size: usize) {
        let mut rng = rand::thread_rng();

        for _i in 0..p_size {
            let mut values: Vec<f64> = vec![];

            for j in 0..c_size {
                values.push(rng.gen_range(self.bounds[j].min..=self.bounds[j].max));
            }
            self.population.push(Chromosome::new(values));
        }
    }

    fn select_parents(&self) -> (&Chromosome, &Chromosome) {
        let upper_bound: f64 = self.population.par_iter().map(|c| c.fitness).sum();

        let mut rng = rand::thread_rng();
        let p_size: usize = self.population.len();

        let index1: usize = rng.gen_range(0..p_size);
        let mut index2: usize = rng.gen_range(0..p_size);

        while index2 == index1 {
            index2 = rng.gen_range(0..p_size);
        }

        let mut parents: (&Chromosome, &Chromosome) =
            (&self.population[index1], &self.population[index2]);

        let mut prob_1: f64 = rng.gen_range(0.0..=upper_bound);
        let mut prob_2: f64 = rng.gen_range(0.0..=upper_bound);
        let mut i: usize = 0;

        // Rotate the roulette
        while (prob_1 > 0.0 || prob_2 > 0.0) && i < p_size {
            if prob_1 > 0.0 {
                prob_1 -= upper_bound - self.population[i].fitness;

                if prob_1 <= 0.0 {
                    parents.0 = &self.population[i];
                }
            } else if prob_2 > 0.0 {
                prob_2 -= upper_bound - self.population[i].fitness;

                if prob_2 <= 0.0 {
                    parents.1 = &self.population[i];
                }
            }
            i += 1;
        }
        parents
    }

    fn crossover(&self, parents: (&Chromosome, &Chromosome)) -> (Chromosome, Chromosome) {
        let length: f64 = parents.0.values.len() as f64;
        let number_of_chromosomes: i32 = (self.crossover_rate * length) as i32;

        let (left_child_1, right_child_2) = parents
            .0
            .values
            .clone()
            .split_inplace_at(number_of_chromosomes as usize);
        let (right_child_1, left_child_2) = parents
            .1
            .values
            .clone()
            .split_inplace_at(number_of_chromosomes as usize);

        let mut left_vec: Vec<f64> = left_child_1.into();
        left_vec.append(&mut left_child_2.into());

        let mut right_vec: Vec<f64> = right_child_1.into();
        right_vec.append(&mut right_child_2.into());

        (Chromosome::new(left_vec), Chromosome::new(right_vec))
    }

    fn compare(c1: &Chromosome, c2: &Chromosome) -> Ordering {
        if c1.fitness < c2.fitness {
            Ordering::Less
        } else if c1.fitness > c2.fitness {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }

    pub fn optimize<F>(&mut self, fitness_function: F) -> Result<Chromosome, ()>
    where
        F: Fn(Vec<f64>) -> f64,
    {
        let mut best: Chromosome = Chromosome::new_empty();

        self.population.iter_mut().for_each(|c| {
            c.fitness = fitness_function(c.values.clone());
        });

        let mut i: usize = 0;
        let mut solutions: Vec<String> = vec![];

        while i < self.max_generations {
            //println!("iteration {:?}: ", i);
            let mut p_size: usize = self.population.len();

            for _j in 0..(p_size / 5) {
                let parents: (&Chromosome, &Chromosome) = self.select_parents();

                let mut new_individuals: (Chromosome, Chromosome) = self.crossover(parents);

                new_individuals.0.fitness = fitness_function(new_individuals.0.values.clone());
                new_individuals.1.fitness = fitness_function(new_individuals.1.values.clone());

                self.population.push(new_individuals.0);
                self.population.push(new_individuals.1);
            }

            let p_newsize: usize = self.population.len();
            let count: usize = p_newsize - p_size;
            p_size = p_newsize;

            //mutate and calculate fitness of each individual of new population
            for id in 1..p_size {
                self.population[id].mutation(self.mutation_rate, &self.bounds);

                self.population[id].fitness = fitness_function(self.population[id].values.clone());
            }

            quicksort_by(&mut self.population, GA::compare);

            //get the best individual
            best = self
                .population
                .first()
                .expect("The population vec is empty")
                .clone();

            for k in 0..count {
                self.population.remove(p_size - 1 - k);
            }

            solutions.push(best.to_string());
            println!("current best is {:?}", best);

            i += 1;
        }

        GA::to_disk::<String>(
            Path::new(&String::from("./src/ode/result/ga_iterations.txt")),
            solutions,
        )
        .unwrap();

        Ok(best)
    }

    pub fn to_disk<P: AsRef<Path>>(path: &Path, data: Vec<String>) -> anyhow::Result<(), Error> {
        let mut file: File = match File::create(path) {
            Ok(f) => f,
            Err(e) => return Err(e.into()),
        };
        for v in &data {
            write!(file, "{ }", v)?;
        }
        Ok(())
    }
}
