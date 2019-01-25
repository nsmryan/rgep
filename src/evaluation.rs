use rand::prelude::*;

use types::*;


pub fn fittest(fitnesses: &Vec<f64>) -> usize {
    let (index, fitness) = 
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

pub fn ga_evaluate<R>(pop: &Pop,
                      eval: &Fn(&Ind, &mut R) -> f64,
                      rng: &mut R) -> Vec<f64>
    where R: Rng {
    let mut fitnesses = Vec::new();

    for ind in pop.0.iter() {
        let fitness = eval(ind, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}
