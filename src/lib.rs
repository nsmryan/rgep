extern crate rand;
extern crate statrs;
#[cfg(test)] extern crate float_cmp;


use rand::prelude::*;

pub mod types;
pub use types::*;

pub mod crossover;
pub use crossover::*;

pub mod rotation;
pub use rotation::*;

pub mod point_mutation;
pub use point_mutation::*;

pub mod ops;
pub use ops::*;

pub mod selection;
pub use selection::*;

pub mod evaluation;
pub use evaluation::*;


pub fn rgep<R: Rng, A: Clone, B: Clone>(params: &Params,
                          context: &Context<A, B>,
                          state: &B,
                          eval_ind: &EvalFunction<A, B, R>,
                          rng: &mut R) -> Pop {
    let mut pop = Pop::create(&params, &context, rng);

    let bits_per_sym = context.bits_per_sym();

    for _ in 0..params.num_gens {
        rotation(&mut pop, params.prob_rotation, rng);
        point_mutation(&mut pop, bits_per_sym, params.prob_mut, rng);
        crossover_one_point(&mut pop, params.ind_size, bits_per_sym, params.prob_one_point_crossover, rng);
        crossover_two_point(&mut pop, params.ind_size, bits_per_sym, params.prob_two_point_crossover, rng);
        let fitnesses = rgep_evaluate(&pop, context, state, eval_ind, rng);
        pop = stochastic_universal_sampling(&pop, fitnesses, rng);
    }

    pop
}


pub fn ga<R: Rng>(params: &GaParams,
                  eval: &Fn(&Ind, &mut R) -> f64,
                  rng: &mut R) -> Pop {
    let mut pop = Pop::create_ga(&params, rng);

    for _ in 0..params.num_gens {
        point_mutation(&mut pop, 8, params.prob_pm, rng);
        crossover_one_point(&mut pop, params.ind_size, 8, params.prob_pc1, rng);
        let fitnesses = ga_evaluate(&pop, eval.clone(), rng);
        pop = stochastic_universal_sampling(&pop, fitnesses, rng);
    }

    pop
}

