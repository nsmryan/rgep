use rand::prelude::*;

use std::iter;
use std::iter::*;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::selection::*;
use crate::evaluation::*;


#[derive(Clone, Copy, PartialEq)]
pub struct GaParams {
    pub ind_size: usize,
    pub pop_size: usize,

    pub num_gens: usize,

    pub elitism: usize,

    pub prob_pm: f64,
    pub prob_pc1: f64,
}

impl Default for GaParams {
    fn default() -> GaParams {
        GaParams {
            ind_size: 100,
            pop_size: 100,
            num_gens: 1000,
            elitism: 0,
            prob_pm: 0.01,
            prob_pc1: 0.6,
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct GaState {
    params: GaParams,
    population: PopU8,
}

impl GaState {
    pub fn create_ga<R>(params: &GaParams, rng: &mut R) -> GaState 
    where R: Rng {
        let mut pop = Vec::with_capacity(params.pop_size);
        for _ in 0..params.pop_size {
            let mut ind_vec = Vec::with_capacity(params.ind_size);
            for _ in 0..params.ind_size {
                ind_vec.push(rng.gen_range(0, 0xFF) as u8);
            }
            pop.push(Ind(ind_vec));
        }

        let population = Pop(pop);
        return GaState { population,
                         params: *params,
        };
    }

    pub fn create_ga_fast(params: &GaParams) -> GaState {
        let ind = Ind(std::iter::repeat(0x0).take(params.ind_size).collect());
        let population = Pop(iter::repeat(ind).take(params.pop_size).collect());

        return GaState { population,
                         params: *params,
        };
    }
}

pub fn ga<R: Rng>(params: &GaParams,
                  eval: &dyn Fn(&Ind<u8>, &mut R) -> f64,
                  rng: &mut R) -> PopU8 {
    let mut state = GaState::create_ga(&params, rng);
    let mut alt_state = GaState::create_ga_fast(&params);

    for _ in 0..params.num_gens {
        point_mutation(&mut state.population, 8, params.prob_pm, rng);
        crossover_one_point(&mut state.population, params.ind_size, 8, params.prob_pc1, rng);
        let fitnesses = evaluate(&state.population, eval.clone(), rng);
        stochastic_universal_sampling(&state.population, &mut alt_state.population, fitnesses, params.elitism, rng);
    }

    state.population
}

