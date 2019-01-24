use rand::prelude::*;

use statrs::distribution::Uniform;

use types::*;


pub fn stochastic_universal_sampling<R: Rng>(pop: &Pop, fitnesses: Vec<f64>, rng: &mut R) -> Pop {
    let offset_scaler = Uniform::new(0.0, 1.0).unwrap().sample(rng);

    select_stochastic_universal(pop, fitnesses, offset_scaler)
}

pub fn select_stochastic_universal(pop: &Pop, fitnesses: Vec<f64>, offset_scaler: f64) -> Pop {
    let num_inds = pop.0.len();
    let mut new_pop = Vec::with_capacity(num_inds);

    let total_fitness = fitnesses.iter().sum::<f64>();
    assert!(total_fitness != 0.0, "Cannot sample when all fitness values are 0.0!");

    let increment = total_fitness / fitnesses.len() as f64;
    let offset = increment * offset_scaler;

    let mut offset = offset;
    let mut accum_fitness = 0.0;
    let mut num_selections = 0;
    let mut ind_index = 0;

    while num_selections < pop.0.len() {
        accum_fitness += fitnesses[ind_index];

        while offset <= accum_fitness {
            new_pop.push(Ind(pop.0[ind_index].0.clone()));
            offset += increment;
            num_selections += 1;
        }
        ind_index += 1;
    }

    Pop(new_pop)
}

