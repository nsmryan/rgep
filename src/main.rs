extern crate rgep;
extern crate rand;
extern crate statrs;
#[cfg(test)] extern crate float_cmp;


use std::collections::HashMap;

use rand::prelude::*;

use rgep::*;


fn main() {
    let terminals =
        vec!(zero_sym(),
             one_sym(),
             two_sym(),
             symbol_sym("x".to_string()));

    let functions =
        vec!(plus_sym(),
             mult_sym(),
             dup_sym(),
             //swap_sym(),
             //drop_sym(),
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

    let mut variables: Variables = HashMap::new();
    variables.insert("x".to_string(), 3.0);

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let mut rng = thread_rng();

    let default = context.default;
    let eval_prog: &EvalFunction<f64, Variables, ThreadRng> = &move |prog: &Program<f64, Variables>, state: &mut Variables, _r: &mut ThreadRng| -> f64 {
        let mut sample_points = Vec::new();
        for x in (0..100).step_by(10) {
            sample_points.push((x as f64, (x * x) as f64));
        }
        let mut fitness = 0.0;
        for point in sample_points {
            state.insert("x".to_string(), point.0);
            let y_actual = prog.eval(state, default);
            fitness += (y_actual - point.1).abs();
        }
        if fitness == 0.0 {
            0.0
        } else {
            1.0 / fitness
        }
    };

    let pop = rgep(&params,
                   &context,
                   &variables,
                   eval_prog,
                   &mut rng);
    for ind in pop.0.iter() {
        let fitness = eval_prog(&ind.compile(&context), &mut variables, &mut rng);
        println!("{} -> {}", ind.to_string(&context), fitness);
    }

    println!("best fitness = {}",
             pop.0.iter()
                  .cloned()
                  .map(|ind| eval_prog(&ind.compile(&context), &mut variables, &mut rng))
                  .fold(0.0 / 0.0, f64::max));
}

