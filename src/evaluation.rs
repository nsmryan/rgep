use rand::prelude::*;

use types::*;


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

pub fn evaluate<R>(pop: &Pop,
                      eval: &Fn(&Ind<u8>, &mut R) -> f64,
                      rng: &mut R) -> Vec<f64>
    where R: Rng {
    let mut fitnesses = Vec::new();

    for ind in pop.0.iter() {
        let fitness = eval(ind, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

