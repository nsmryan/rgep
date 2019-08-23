use rand::prelude::*;

use std::iter;
use std::iter::*;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::rotation::*;
use crate::selection::*;
use crate::evaluation::*;


#[derive(Clone)]
pub struct RgepParams {
    pub prob_mut: f64,
    pub prob_one_point_crossover: f64,
    pub prob_two_point_crossover: f64,
    pub prob_rotation: f64,

    pub pop_size: usize,
    pub ind_size: usize,

    pub elitism: usize,

    pub num_gens: usize,
}

impl Default for RgepParams {
    fn default() -> Self {
        RgepParams {
            prob_mut: 0.001,
            prob_one_point_crossover: 0.6,
            prob_two_point_crossover: 0.6,
            prob_rotation: 0.01,
            pop_size: 25,
            ind_size: 100,
            elitism: 1,
            num_gens: 100,
        }
    }
}

pub fn create_rgep<A, R, B>(params: &RgepParams, context: &Context<A, B>, rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
    let mut pop = Vec::with_capacity(params.pop_size);

    let bits_needed = context.bits_per_sym();
    assert!(bits_needed <= 8, "This implementation does not currently support multiple byte symbols");

    let range = 2_u32.pow(bits_needed as u32);

    for _ in 0..params.pop_size {
        let mut ind_vec = Vec::with_capacity(params.ind_size);
        for _ in 0..params.ind_size {
            ind_vec.push(rng.gen_range(0, range) as u8);
        }
        pop.push(Ind(ind_vec));
    }

    Pop(pop)
}

pub fn create_rgep_fast<A, B>(params: &RgepParams, context: &Context<A, B>) -> Pop 
    where A: Clone, B: Clone {
    let ind = Ind(iter::repeat(0x0).take(params.ind_size).collect());
    Pop(iter::repeat(ind).take(params.pop_size).collect())
}


pub fn rgep_evaluate<R, A, B>(pop: &Pop,
                              context: &Context<A, B>,
                              state: &B,
                              eval_prog: &EvalFunction<A, B, R>,
                              rng: &mut R) -> Vec<f64>
    where R: Rng,
          A: Clone,
          B: Clone {
    let mut fitnesses = Vec::new();

    let mut prog = Program(Vec::with_capacity(pop.0[0].0.len()));

    for ind in pop.0.iter() {
        let mut local_state = state.clone();
        ind.compile_to(context, &mut prog);
        let fitness = eval_prog(&prog, &mut local_state, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

pub fn rgep<R, A, B>(params: &RgepParams,
                     context: &Context<A, B>,
                     state: &B,
                     eval_ind: &EvalFunction<A, B, R>,
                     rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
    let mut pop = Box::new(create_rgep(&params, &context, rng));
    let mut alt_pop = Box::new(create_rgep_fast(&params, &context));

    let bits_per_sym = context.bits_per_sym();

    for _ in 0..params.num_gens {
        rotation(&mut pop, params.prob_rotation, rng);
        point_mutation(&mut pop, bits_per_sym, params.prob_mut, rng);
        crossover_one_point(&mut pop, params.ind_size, bits_per_sym, params.prob_one_point_crossover, rng);
        crossover_two_point(&mut pop, params.ind_size, bits_per_sym, params.prob_two_point_crossover, rng);
        let fitnesses = rgep_evaluate(&pop, context, state, eval_ind, rng);
        stochastic_universal_sampling(&pop, &mut alt_pop, fitnesses, params.elitism, rng);

        std::mem::swap(&mut pop, &mut alt_pop);
    }

    *pop
}

