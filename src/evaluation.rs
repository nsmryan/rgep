use rand::prelude::*;

use types::*;


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
    pub population: Rc<RefCell<Pop8>>,
    pub eval: Eval<Ind<u8>, R>,
}

pub fn evaluate_stage<S, R>(getter: Getter<S, EvalState>) -> Stage<S, R>
    where R: Rng,
          S: 'static {
    let f: Rc<dyn Fn(&S, &mut R)> = Rc::new(move |state, rng| {
        let mut eval_state = getter(state);
        evaluate(&mut eval_state.population.borrow_mut(),
                 eval_state.eval,
                 rng);
    });

    return f;
}

pub fn evaluate<R>(pop: &PopU8,
                   eval: &dyn Fn(&Ind<u8>, &mut R) -> f64,
                   rng: &mut R) -> Vec<f64>
    where R: Rng {
    let mut fitnesses = Vec::new();

    for ind in pop.0.iter() {
        let fitness = eval(ind, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

