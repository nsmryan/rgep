pub mod context;

use std::rc::Rc;
use std::iter;
use std::iter::*;

use rand::prelude::*;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::rotation::*;
use crate::selection::*;

use domains::symbols::*;

use context::*;


pub type EvalFunction<A, B, R> = dyn Fn(&Program<A, B>, &mut B, &mut R) -> f64;


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

pub fn create_rgep<A, R, B>(params: &RgepParams, context: &Context<A, B>, rng: &mut R) -> PopU8 
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

pub fn create_rgep_fast(params: &RgepParams) -> PopU8 {
    let ind = Ind(iter::repeat(0x0).take(params.ind_size).collect());
    Pop(iter::repeat(ind).take(params.pop_size).collect())
}


pub fn rgep_evaluate<R, A, B>(pop: &PopU8,
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
        context.compile_to(&ind, &mut prog);
        let fitness = eval_prog(&prog, &mut local_state, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

pub fn rgep<R, A, B>(params: &RgepParams,
                     context: &Context<A, B>,
                     state: &B,
                     eval_ind: &EvalFunction<A, B, R>,
                     rng: &mut R) -> PopU8
    where R: Rng, A: Clone, B: Clone {
    let mut pop = create_rgep(&params, &context, rng);
    let mut alt_pop = create_rgep_fast(&params);

    let bits_per_sym = context.bits_per_sym();

    for _ in 0..params.num_gens {
        //rotation(&mut pop, params.prob_rotation, rng);
        //point_mutation(&mut pop, bits_per_sym, params.prob_mut, rng);
        //crossover_one_point(&mut pop, params.ind_size, bits_per_sym, params.prob_one_point_crossover, rng);
        //crossover_two_point(&mut pop, params.ind_size, bits_per_sym, params.prob_two_point_crossover, rng);
        //let fitnesses = rgep_evaluate(&pop, context, state, eval_ind, rng);
        //stochastic_universal_sampling(&pop, &mut alt_pop, fitnesses, params.elitism, rng);

        //std::mem::swap(&mut pop, &mut alt_pop);
    }

    pop
}

#[test]
fn test_point_mutation_flips_bits() {
    use domains::arith::{plus_sym, one_sym, zero_sym, two_sym};

    let terminals: Vec<Sym<f64, ()>> =
        vec!(zero_sym(), one_sym(), two_sym());

    let functions =
        vec!(plus_sym());

    let num_inds  = 200;
    let num_words = 200;

    let params: RgepParams = RgepParams {
        prob_mut: 0.001,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.9,
        pop_size: num_inds,
        ind_size: num_words,
        num_gens: 100,
        elitism: 0,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    let mut rng = thread_rng();

    let mut pop = create_rgep(&params, &context, &mut rng);
    let bits_per_sym = context.bits_per_sym();
    point_mutation(&mut pop, bits_per_sym, params.prob_mut, &mut rng);

    let mut num_ones = 0;
    for ind in pop.0 {
        for code_word in ind.0 {
            for bit_index in 0..bits_per_sym {
                if code_word & (1 << bit_index) != 0 {
                    num_ones += 1;
                }
            }
        }
    }

    println!("num ones = {}", num_ones);
    let percent_ones = num_ones as f64 / (num_inds as f64 * num_words as f64 * bits_per_sym as f64);
    println!("percent ones = {}", percent_ones);
    // NOTE this does a statically likely test within a unit test, which may be a bad idea.
    assert!((percent_ones - 0.5).abs() < 0.005, format!("Percent ones was expected to be 0.5, but was {}", percent_ones));
}

