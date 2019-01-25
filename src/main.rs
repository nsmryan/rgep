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
             two_sym());
             

    let functions =
        vec!(plus_sym(),
             mult_sym(),
             sub_sym(),
             //div_sym(),

             dup_sym(),
             swap_sym(),
             drop_sym(),

             printout(),
             //load_mem(),
             //store_mem(),
             store_a(),
             load_a(),
             //store_b(),
             //load_b(),
             );

    let params: Params = Params {
        prob_mut: 0.01,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.01,
        pop_size: 100,
        ind_size: 30,
        num_gens: 50000,
        elitism: 1,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    let mut instr_state = Default::default();

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let mut rng = thread_rng();

    let default = context.default.clone();
    let eval_prog: &EvalFunction<f64, InstrState, ThreadRng> =
        &move |prog: &Program<f64, InstrState>, state: &mut InstrState, _r: &mut ThreadRng| -> f64 {
            let mut penalty = 0.0;

            prog.eval(state, default.clone());
            let amount_error: f64 =
                state.output.iter()
                            .zip(vec!(0.0, 1.0, 2.0, 3.0, 4.0))
                            .map(|(a, b)| (a - b).abs())
                            .sum();

            penalty += 5.0 * (5.0 - state.output.len() as f64).abs();

            penalty += amount_error;

            if penalty == 0.0 {
                penalty = 0.0001;
            }

            1.0 / penalty
        };

    let pop = rgep(&params,
                   &context,
                   &instr_state,
                   eval_prog,
                   &mut rng);

    let mut fitnesses = Vec::new();
    let mut local_states = Vec::new();
    for ind in pop.0.iter() {
        let mut local_state = instr_state.clone();
        let fitness = eval_prog(&ind.compile(&context), &mut local_state, &mut rng);
        fitnesses.push(fitness);
        println!("local_state = {:?}", local_state);
        local_states.push(local_state);
        println!("{} -> {}", ind.to_string(&context), fitness);
    }

    let index_fittest = fittest(&fitnesses);
    let fittest = pop.0[index_fittest].clone();
    let fitness = fitnesses[index_fittest];

    println!("best fitness    = {}", fitness);
    println!("best individual = {:?}", pop.0[index_fittest].to_string(&context));
    println!("best output = {:?}", local_states[index_fittest]);
}

fn main_expr() {
    let terminals =
        vec!(const_expr(0.0),
             const_expr(1.0),
             const_expr(2.0),
             var_expr("x".to_string()));

    let functions =
        vec!(add_expr(),
             mult_expr(),
             dup_sym(),
             swap_sym(),
             drop_sym(),
             );

    let params: Params = Params {
        prob_mut: 0.005,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.001,
        pop_size: 100,
        ind_size: 1000,
        num_gens: 100,
        elitism: 1,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: Arith::Const(0.0),
    };

    let mut variables: Variables = HashMap::new();
    variables.insert("x".to_string(), 3.0);

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let mut rng = thread_rng();

    let default = context.default.clone();
    let eval_prog: &EvalFunction<Arith, Variables, ThreadRng> =
        &move |prog: &Program<Arith, Variables>, state: &mut Variables, _r: &mut ThreadRng| -> f64 {
            let mut sample_points = Vec::new();
            for x in (0..100).step_by(10) {
                sample_points.push((x as f64, (x * x) as f64));
            }
            let mut fitness = 0.0;
            for point in sample_points {
                state.insert("x".to_string(), point.0);
                let y_actual = prog.eval(state, default.clone()).eval(state);
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

    let mut fitnesses = Vec::new();
    for ind in pop.0.iter() {
        let fitness = eval_prog(&ind.compile(&context), &mut variables, &mut rng);
        fitnesses.push(fitness);
        println!("{} -> {}", ind.to_string(&context), fitness);
    }

    let index_fittest = fittest(&fitnesses);
    let fittest = pop.0[index_fittest].clone();
    let fitness = fitnesses[index_fittest];

    println!("best fitness    = {}", fitness);
    println!("best individual = {:?}", pop.0[index_fittest].to_string(&context));
    println!("infix = {:?}", pop.0[index_fittest].eval(&context, &mut variables).simplify().to_string_infix());
}

