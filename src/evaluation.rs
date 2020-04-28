use std::rc::Rc;
use std::cell::RefCell;

use rand::prelude::*;

use types::*;
use stage::*;


pub type Eval<Ind, R> = Rc<dyn Fn(&Ind, &mut R) -> f64>;

pub fn fittest(fitnesses: &Vec<f64>) -> usize {
    let (index, _fitness) = 
        fitnesses.iter()
                  .enumerate()
                  .fold((0, 0.0), |(best_index, best_fitness), (index, fitness)| {
                     if *fitness > best_fitness {
                         (index, *fitness)
                     } else {
                         (best_index, best_fitness)
                     }
                   });
    index
}

pub struct EvalState<R> {
    pub population: Rc<RefCell<PopU8>>,
    pub eval: Eval<Ind<u8>, R>,
    pub fitnesses: Rc<RefCell<Vec<f64>>>,
}

impl<R> EvalState<R> {
    pub fn new(population: Rc<RefCell<PopU8>>,
               eval: Eval<Ind<u8>, R>,
               fitnesses: Rc<RefCell<Vec<f64>>>) -> EvalState<R> {
        return EvalState {
            population,
            eval,
            fitnesses,
        };
    }
}

pub fn evaluate_stage<S, R>(getter: Getter<S, EvalState<R>>) -> Stage<S, R>
    where R: Rng + 'static,
          S: 'static {
    let f: Rc<dyn Fn(&S, &mut R)> = Rc::new(move |state, rng| {
        let mut eval_state = getter(state);
        evaluate(&mut eval_state.population.borrow_mut(),
                 eval_state.eval.clone(),
                 &mut eval_state.fitnesses.borrow_mut(),
                 rng);
    });

    return f;
}

pub fn evaluate<R>(pop: &PopU8,
                   eval: Eval<IndU8, R>,
                   fitnesses: &mut Vec<f64>,
                   rng: &mut R)
    where R: Rng {
    for (index, ind) in pop.0.iter().enumerate() {
        fitnesses[index] = eval(ind, rng);
    }
}

