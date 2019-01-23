extern crate rgep;
extern crate rand;
extern crate statrs;
#[cfg(test)] extern crate float_cmp;


use rand::prelude::*;
use rand::distributions::Distribution;

use statrs::distribution::{Uniform};

use rgep::*;


fn main() {
    let terminals =
        vec!(zero_sym::<()>(), one_sym::<()>(), two_sym::<()>());

    let functions =
        vec!(plus_sym::<()>(),
             mult_sym::<()>(),
             //dup_sym::<()>(),
             //swap_sym::<()>(),
             //drop_sym::<()>(),
             );

    let params: Params = Params {
        prob_mut: 0.005,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.001,
        pop_size: 25,
        ind_size: 25,
        num_gens: 200,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let mut rng = thread_rng();

    let context_clone = context.clone();
    let eval_ind: &EvalFunction<(), ThreadRng> = &move |ind: &Ind, state: &mut (), r: &mut ThreadRng| -> f64 {
        ind.eval(&context_clone, &mut ())
    };

    let pop = rgep(&params,
                   &context,
                   &(),
                   eval_ind,
                   &mut rng);
    for ind in pop.0.iter() {
        let fitness = eval_ind(&ind, &mut (), &mut rng);
        println!("{} -> {}", ind.to_string(&context), fitness);
    }

    println!("best fitness = {}",
             pop.0.iter()
                  .cloned()
                  .map(|ind| eval_ind(&ind, &mut (), &mut rng))
                  .fold(0.0 / 0.0, f64::max));
}

