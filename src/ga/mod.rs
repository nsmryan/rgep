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

#[derive(Clone, PartialEq)]
pub struct GaState {
    params: GaParams,
    population: Rc<RefCell<Pop<u8>>>,
    eval: Eval<Ind<u8>, R>,
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
        return GaState { population: Rc::new(RefCell::new(population)),
                         params: *params,
        };
    }

    pub fn create_ga_fast(params: &GaParams) -> GaState {
        let ind = Ind(std::iter::repeat(0x0).take(params.ind_size).collect());
        let population = Pop(iter::repeat(ind).take(params.pop_size).collect());

        return GaState { population: Rc::new(RefCell::new(population)),
                         params: *params,
        };
    }
}

pub fn ga<R: Rng>(params: &GaParams,
                  eval: &dyn Fn(&Ind<u8>, &mut R) -> f64,
                  rng: &mut R) -> Rc<RefCell<PopU8>> {
    let state = GaState::create_ga(&params, rng);
    let alt_state = GaState::create_ga_fast(&params);

    let pm_stage: Stage<GaState, R> = point_mutation_stage(Rc::new(|state: &GaState| {
        return PmState::new(state.population.clone(), state.params.prob_pm, 8);
    }));

    let cross_stage: Stage<GaState, R> = crossover_stage(Rc::new(|state: &GaState| {
        return CrossoverState::new(state.population.clone(), state.params.prob_pc1);
    }));

    let eval_stage: Stage<GaState, R> = eval_stage(Rc::new(|state: &GaState| {
        return EvalState::new(state.population.clone(), state.eval);
    }));

    for _ in 0..params.num_gens {
        pm_stage(&state, rng);
        cross_stage(&state, rng);
        //let fitnesses = evaluate(&state.population, eval.clone(), rng);
        //stochastic_universal_sampling(&state.population, &mut alt_state.population, fitnesses, params.elitism, rng);
    }

    return state.population;
}

