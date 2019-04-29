extern crate rand;
extern crate statrs;
extern crate im;
extern crate num;
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

#[test]
fn test_swap_with_boxes() {
    let mut first: Box<Vec<usize>> = Box::new((0..10).collect());
    let mut second = Box::new((0..10).collect());
    let mut tmp = Box::new(Vec::new());

    first[0] = 1234;

    tmp = first;
    first = second;
    second = tmp;
    assert!(first[0] == 0);
    assert!(second[0] == 1234);

    tmp = first;
    first = second;
    second = tmp;
    assert!(first[0] == 1234);
    assert!(second[0] == 0);
}

pub fn rgep<R, A, B>(params: &Params,
                     context: &Context<A, B>,
                     state: &B,
                     eval_ind: &EvalFunction<A, B, R>,
                     rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
    let mut pop = Box::new(Pop::create(&params, &context, rng));
    let mut alt_pop = Box::new(Pop::create_fast(&params, &context));

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


pub fn ga<R: Rng>(params: &GaParams,
                  eval: &Fn(&Ind, &mut R) -> f64,
                  rng: &mut R) -> Pop {
    let mut pop = Pop::create_ga(&params, rng);
    let mut alt_pop = Pop::create_ga_fast(&params);

    for _ in 0..params.num_gens {
        point_mutation(&mut pop, 8, params.prob_pm, rng);
        crossover_one_point(&mut pop, params.ind_size, 8, params.prob_pc1, rng);
        let fitnesses = ga_evaluate(&pop, eval.clone(), rng);
        stochastic_universal_sampling(&pop, &mut alt_pop, fitnesses, params.elitism, rng);
    }

    pop
}

