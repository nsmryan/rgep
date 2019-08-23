use rand::prelude::*;

use std::iter;
use std::iter::*;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::selection::*;
use crate::evaluation::*;


#[derive(Clone, PartialEq)]
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

pub fn create_ga<R>(params: &GaParams, rng: &mut R) -> Pop 
where R: Rng {
    let mut pop = Vec::with_capacity(params.pop_size);
    for _ in 0..params.pop_size {
        let mut ind_vec = Vec::with_capacity(params.ind_size);
        for _ in 0..params.ind_size {
            ind_vec.push(rng.gen_range(0, 0xFF) as u8);
        }
        pop.push(Ind(ind_vec));
    }

    Pop(pop)
}

pub fn create_ga_fast(params: &GaParams) -> Pop {
    let ind = Ind(std::iter::repeat(0x0).take(params.ind_size).collect());
    Pop(iter::repeat(ind).take(params.pop_size).collect())
}

pub fn ga<R: Rng>(params: &GaParams,
                  eval: &Fn(&Ind, &mut R) -> f64,
                  rng: &mut R) -> Pop {
    let mut pop = create_ga(&params, rng);
    let mut alt_pop = create_ga_fast(&params);

    for _ in 0..params.num_gens {
        point_mutation(&mut pop, 8, params.prob_pm, rng);
        crossover_one_point(&mut pop, params.ind_size, 8, params.prob_pc1, rng);
        let fitnesses = evaluate(&pop, eval.clone(), rng);
        stochastic_universal_sampling(&pop, &mut alt_pop, fitnesses, params.elitism, rng);
    }

    pop
}

