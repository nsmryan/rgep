use std::cmp::Ordering;
use std::iter;

use num::traits::Pow;

use rand::prelude::*;

use statrs::distribution::Uniform;

use types::*;


pub fn k_elite(fitnesses: &Vec<f64>, num_elite: usize) -> Vec<usize> {
    // elitism- give a certain number of individuals a free pass to the next generation
    let mut elite_indices: Vec<usize> = Vec::new();
    if num_elite > 0 {
        // pair fitnesses with indices
        let mut elite_paired: Vec<(usize, &f64)> = fitnesses.iter().enumerate().collect();
        // sort paired fitnesses by fitness
        elite_paired .sort_unstable_by(|(_index, fitness), (_index_other, fitness_other)| {
            if fitness > fitness_other {
                Ordering::Less
            } else if fitness > fitness_other {
                Ordering::Greater
            } else { Ordering::Equal }
        });
        // add the k most elite individual's indices to the elite_indices vec
        elite_indices.extend(elite_paired.iter().take(num_elite).map(|(index, _)| index));
    }

    elite_indices
}

pub fn tournament_selection<R: Rng>(pop: &Pop, new_pop: &mut Pop, fitnesses: Vec<f64>, prob: f64, tourn_size: usize, elitism: usize, rng: &mut R) {
    let winner_rng = Uniform::new(0.0, 1.0).unwrap();

    let num_inds = pop.0.len();

    let mut elite_indices = k_elite(&fitnesses, elitism);

    let mut num_selections = 0;

    let prob_indices = (0..tourn_size).map(|_index| prob * (1.0 - prob).pow(tourn_size as i32)).collect::<Vec<f64>>();

    while num_selections < num_inds {
        // NOTE consider re-using this vector. benchmark for comparison
        let mut tourny = iter::repeat_with(|| {
            let index = rng.gen_range(0, pop.0[0].0.len());
            let ind = &pop.0[index];
            let fitness = fitnesses[index];
            (ind, fitness)
        }).collect::<Vec<(&Ind<u8>, f64)>>();

        tourny.sort_unstable_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let pos_rng = rng.sample(winner_rng);

        let winner_index = prob_indices.iter().position(|p| pos_rng <= *p).unwrap();

        if let Some(elite_index) = elite_indices.iter().position(|a| *a == winner_index) {
            elite_indices.swap_remove(elite_index);
        }

        num_selections += 1;
    }

    for elite_index in elite_indices {
        new_pop.0[elite_index].0.clear();
        new_pop.0[elite_index].0.extend(pop.0[num_selections].0.iter());
    }
}

pub fn stochastic_universal_sampling<R: Rng>(pop: &Pop, new_pop: &mut Pop, fitnesses: Vec<f64>, elitism: usize, rng: &mut R) {
    let offset_scaler = Uniform::new(0.0, 1.0).unwrap().sample(rng);

    select_stochastic_universal(pop, new_pop, fitnesses, elitism, offset_scaler);
}

pub fn select_stochastic_universal_naive(pop: &Pop, fitnesses: Vec<f64>, elitism: usize, offset_scaler: f64) -> Pop {
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

    // elitism- give a certain number of individuals a free pass to the next generation
    let mut elite_indices = k_elite(&fitnesses, elitism);

    while num_selections < pop.0.len() {
        accum_fitness += fitnesses[ind_index];

        // if we are going to skip this individual, check if they are elite
        if offset > accum_fitness {
            // an individual is elite if its index is in the elite_indices vec
            let elite_pos = elite_indices.iter().position(|index| *index == ind_index);
            if elite_pos.is_some() {
                // put the individual in the new population
                new_pop.push(Ind(pop.0[ind_index].0.clone()));
                // remove the individual from the elite array, just to make it smaller
                // for subsequent checks.
                elite_indices.swap_remove(elite_pos.unwrap());
                num_selections += 1;
            }
        }

        while offset <= accum_fitness {
            new_pop.push(Ind(pop.0[ind_index].0.clone()));
            offset += increment;
            num_selections += 1;
        }

        ind_index += 1;
    }

    Pop(new_pop)
}

pub fn select_stochastic_universal(pop: &Pop, new_pop: &mut Pop, fitnesses: Vec<f64>, elitism: usize, offset_scaler: f64) {
    let total_fitness = fitnesses.iter().sum::<f64>();
    assert!(total_fitness != 0.0, "Cannot sample when all fitness values are 0.0!");

    let increment = total_fitness / fitnesses.len() as f64;
    assert!(increment.is_normal(), format!("Selection cannot work with {} increment!", increment));
    let offset = increment * offset_scaler;

    let mut offset = offset;
    let mut accum_fitness = 0.0;
    let mut num_selections = 0;
    let mut ind_index = 0;

    // elitism- give a certain number of individuals a free pass to the next generation
    let mut elite_indices = k_elite(&fitnesses, elitism);

    while num_selections < pop.0.len() {
        accum_fitness += fitnesses[ind_index];

        // if we are going to skip this individual, check if they are elite
        if offset > accum_fitness {
            // an individual is elite if its index is in the elite_indices vec
            let elite_pos = elite_indices.iter().position(|index| *index == ind_index);
            if elite_pos.is_some() {
                // put the individual in the new population
                new_pop.0[ind_index].0.clear();
                new_pop.0[ind_index].0.extend(pop.0[ind_index].0.iter());
                // remove the individual from the elite array, just to make it smaller
                // for subsequent checks.
                elite_indices.swap_remove(elite_pos.unwrap());
                num_selections += 1;
            }
        }

        while offset <= accum_fitness {
            new_pop.0[ind_index].0.clear();
            new_pop.0[ind_index].0.extend(pop.0[ind_index].0.iter());
            offset += increment;
            num_selections += 1;
        }

        ind_index += 1;
    }
}

