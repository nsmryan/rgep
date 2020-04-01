use std::rc::Rc;
use std::iter;
use std::iter::*;
use std::cell::RefCell;
use std::boxed::Box;

use rand::prelude::*;

use num::PrimInt;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::selection::*;
use crate::evaluation::*;
use crate::stage::Stage;


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

#[derive(Clone)]
pub struct GaState<R> {
    params: GaParams,
    population: Rc<RefCell<PopU8>>,
    alt_population: Rc<RefCell<PopU8>>,
    eval: Eval<IndU8, R>,
    fitnesses: Rc<RefCell<Vec<f64>>>,
}

pub fn populationU8<R: Rng>(pop_size: usize, ind_size: usize, rng: &mut R) -> PopU8 {
    let mut pop = Vec::with_capacity(pop_size);
    for _ in 0..pop_size {
        let mut ind_vec = Vec::with_capacity(ind_size);
        for _ in 0..ind_size {
            ind_vec.push(rng.gen_range(0, 0xFF) as u8);
        }
        pop.push(Ind(ind_vec));
    }

    let population = Pop(pop);

    return population;
}

pub fn populationU8Zeros(pop_size: usize, ind_size: usize) -> PopU8 {
    let ind = Ind(std::iter::repeat(0x0).take(ind_size).collect());
    let population = Pop(iter::repeat(ind).take(pop_size).collect());

    return population;
}

impl<R: Rng> GaState<R> {
    pub fn create_ga(params: &GaParams, eval: Eval<IndU8, R>, rng: &mut R) -> GaState<R> {
        let population = populationU8(params.pop_size, params.ind_size, rng);
        let alt_population = populationU8Zeros(params.pop_size, params.ind_size);
        let fitnesses = vec![0.0; params.pop_size];

        return GaState { population: Rc::new(RefCell::new(population)),
                         alt_population: Rc::new(RefCell::new(alt_population)),
                         eval,
                         params: *params,
                         fitnesses: Rc::new(RefCell::new(fitnesses)),
        };
    }
}

pub fn ga<R>(params: &GaParams,
                  eval: Eval<IndU8, R>,
                  rng: &mut R) -> Rc<RefCell<PopU8>> 
    where R: Rng + 'static {
    let state = GaState::create_ga(&params, eval, rng);

    let pm_stage: Stage<GaState<R>, R> = point_mutation_stage(Rc::new(|state: &GaState<R>| {
        return PmState::new(state.population.clone(), state.params.prob_pm, 8);
    }));

    let cross_stage: Stage<GaState<R>, R> = crossover_stage(Rc::new(|state: &GaState<R>| {
        return CrossoverState::new(state.population.clone(), state.params.prob_pc1);
    }));

    let eval_stage: Stage<GaState<R>, R> = evaluate_stage(Rc::new(|state: &GaState<R>| {
        return EvalState::new(state.population.clone(), state.eval.clone(), state.fitnesses.clone());
    }));

    let sus_stage : Stage<GaState<R>, R> = sus_stage(Rc::new(|state: &GaState<R>| {
        return SusState::new(state.population.clone(), state.alt_population.clone(), state.fitnesses.clone(), state.params.elitism);
    }));

    for _ in 0..params.num_gens {
        pm_stage(&state, rng);
        cross_stage(&state, rng);
        eval_stage(&state, rng);
        sus_stage(&state, rng);
    }

    return state.population;
}

