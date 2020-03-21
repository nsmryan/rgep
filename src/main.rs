extern crate rgep;
extern crate rand;
extern crate statrs;
#[cfg(test)] extern crate float_cmp;

use std::collections::HashMap;

use rand::prelude::*;

use rgep::*;
use rgep::context::*;
use domains::arith::{mult_expr, add_expr, const_expr, var_expr};


fn popcount(word: u32) -> u32 {
    let mut bit_sum = 0;
    let mut bits = word;

    for _ in 0..32 {
        bit_sum += bits & 1;
        bits = bits >> 1;
    }

    bit_sum
}

#[test]
fn test_popcount() {
    assert_eq!(popcount(0x0001), 1);
    assert_eq!(popcount(0x1111), 4);
    assert_eq!(popcount(0xA5A5), 8);
}

fn main() {
    main_popcount();
}

fn main_popcount() {
    let terminals: Vec<Sym<u32, u32>> =
        vec!(zero_sym(),
             one_sym(),
             two_sym(),
             push_context_sym(),
             );

    let functions =
        vec!(//plus_sym(),
             //mult_sym(),
             //sub_sym(),
             //mod_sym(),

             and_sym(),
             or_sym(),
             xor_sym(),
             not_sym(),

             dup_sym(),
             swap_sym(),
             drop_sym(),
             );

    let params: RgepParams = RgepParams {
        prob_mut: 0.005,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.01,
        pop_size: 50,
        ind_size: 10,
        num_gens: 1000,
        elitism: 1,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0,
    };

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let default = context.default.clone();
    let eval_prog: &EvalFunction<u32, u32, ThreadRng> =
        &move |prog: &Program<u32, u32>, _state: &mut u32, _r: &mut ThreadRng| -> f64 {
            let mut rng = thread_rng();
            let mut penalty: f64 = 1.0;

            for _ in 0..100 {
                let mut word: u32 = rng.gen();
                let mut stack = Vec::new();
                stack.push(word);
                let mut result = prog.eval_with_stack(&mut word, default.clone() as u32, &mut stack);
                if stack.len() > 0 {
                    result = stack[0];
                }
                penalty += (popcount(word) as f64 - result as f64).abs();
            }

            1.0 / penalty
        };

    let pop = rgep(&params,
                   &context,
                   &0,
                   eval_prog,
                   &mut thread_rng());

    let mut rng = thread_rng();

    let mut variables: Variables<u32> = HashMap::new();
    variables.insert("x".to_string(), 3);

    let mut fitnesses = Vec::new();
    for ind in pop.0.iter() {
        let fitness = eval_prog(&context.compile(&ind), &mut 0, &mut rng);
        fitnesses.push(fitness);
        println!("{} -> {}", context.to_string(&ind), fitness);
    }

    let index_fittest = fittest(&fitnesses);
    //let fittest = pop.0[index_fittest].clone();
    let fitness = fitnesses[index_fittest];

    println!("best fitness    = {}", fitness);
    println!("best individual = {:?}", context.to_string(&pop.0[index_fittest]));

    let words = vec!(0xA5A5, 0x1234, 0x1010, 0x0001);
    for mut word in words {
        let mut stack = vec!(word);
        let mut prog = Program(Vec::with_capacity(pop.0[0].0.len()));
        context.compile_to(&pop.0[index_fittest], &mut prog);
        let result = prog.eval_with_stack(&mut word, default.clone() as u32, &mut stack);
        println!("Expected {}, was {}", popcount(word), result);
    }
}

fn main_expr() {
    let terminals: Vec<Sym<Arith<u32>, Variables<u32>>> =
        vec!(const_expr(0),
             const_expr(1),
             const_expr(2),
             var_expr("x".to_string()));

    let functions =
        vec!(add_expr(),
             mult_expr(),
             dup_sym(),
             swap_sym(),
             drop_sym(),
             );

    let params: RgepParams = RgepParams {
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
        default: Arith::Const(0),
    };

    let mut variables: Variables<u32> = HashMap::new();
    variables.insert("x".to_string(), 3);

    println!("bits = {}", context.bits_per_sym());
    println!("bytes = {}", context.bytes_per_sym());
    
    let mut rng = thread_rng();

    let default = context.default.clone();
    let eval_prog: &EvalFunction<Arith<u32>, Variables<u32>, ThreadRng> =
        &move |prog: &Program<Arith<u32>, Variables<u32>>, state: &mut Variables<u32>, _r: &mut ThreadRng| -> f64 {
            let mut sample_points: Vec<(u32, u32)> = Vec::new();
            for x in (0..100).step_by(10) {
                sample_points.push((x as u32, (x * x) as u32));
            }
            let mut fitness = 0.0;
            for point in sample_points {
                state.insert("x".to_string(), point.0);
                let y_actual: u32 = prog.eval(state, default.clone()).eval(state);
                fitness += (y_actual as f64 - point.1 as f64).abs();
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
        let fitness = eval_prog(&context.compile(&ind), &mut variables, &mut rng);
        fitnesses.push(fitness);
        println!("{} -> {}", context.to_string(&ind), fitness);
    }

    let index_fittest = fittest(&fitnesses);
    //let fittest = pop.0[index_fittest].clone();
    let fitness = fitnesses[index_fittest];

    println!("best fitness    = {}", fitness);
    println!("best individual = {:?}", context.to_string(&pop.0[index_fittest]));
    println!("infix = {:?}", context.eval(&pop.0[index_fittest], &mut variables).simplify().to_string_infix());
}

