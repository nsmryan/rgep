use rand::prelude::*;

use statrs::distribution::{Uniform, Geometric};

use im::vector::Vector;

use types::*;
use ops::*;

#[cfg(test)]
use crate::rgep::*;

#[cfg(test)]
use crate::ga::*;


pub fn point_mutation_naive<R: Rng>(pop: &mut Pop, bits_used: usize, pm: f64, rng: &mut R) {
    for ind in pop.0.iter_mut() {
        point_mutate_naive(ind, bits_used, pm, rng);
    }
}

pub fn point_mutate_naive<R: Rng>(ind: &mut Ind<u8>, bits_used: usize, pm: f64, rng: &mut R) {
    let sampler = Uniform::new(0.0, 1.0).unwrap();

    for loc in ind.0.iter_mut() {
        for bit_index in 0..bits_used {
            if sampler.sample(rng) < pm {
                *loc = *loc ^ (1 << bit_index);
            }
        }
    }
}

pub fn point_mutation<R: Rng>(pop: &mut Pop, bits_used: usize, pm: f64, rng: &mut R) {
    for ind in pop.0.iter_mut() {
        point_mutate(ind, bits_used, pm, rng);
    }
}

pub fn point_mutate<R: Rng>(ind: &mut Ind<u8>, bits_used: usize, pm: f64, rng: &mut R) {
    let ind_len_bits = ind.0.len() * bits_used;

    let sampler = Geometric::new(pm).unwrap();

    let mut next_loc_bits = sampler.sample(rng) as usize;
    
    while next_loc_bits < ind_len_bits {
        let next_loc = next_loc_bits / bits_used;
        let bit_index = next_loc_bits % bits_used;

        let word = ind.0[next_loc];

        ind.0[next_loc] = word ^ (1 << bit_index);

        next_loc_bits += sampler.sample(rng) as usize;
    }
}

pub fn point_mutate_im<R: Rng>(ind: &mut Vector<u8>, bits_used: usize, pm: f64, rng: &mut R) {
    let ind_len_bits = ind.len() * bits_used;

    let sampler = Geometric::new(pm).unwrap();

    let mut next_loc_bits = sampler.sample(rng) as usize;
    
    while next_loc_bits < ind_len_bits {
        let next_loc = next_loc_bits / bits_used;
        let bit_index = next_loc_bits % bits_used;

        let word = ind.get_mut(next_loc).unwrap();

        *word = *word ^ (1 << bit_index);

        next_loc_bits += sampler.sample(rng) as usize;
    }
}

#[test]
fn test_point_mutation_flips_bits() {
    let terminals: Vec<Sym<f64, ()>> =
        vec!(zero_sym(), one_sym(), two_sym());

    let functions =
        vec!(plus_sym());

    let num_inds  = 200;
    let num_words = 200;

    let params: RgepParams = RgepParams {
        prob_mut: 0.001,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.9,
        pop_size: num_inds,
        ind_size: num_words,
        num_gens: 100,
        elitism: 0,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    let mut rng = thread_rng();

    let mut pop = create_rgep(&params, &context, &mut rng);
    let bits_per_sym = context.bits_per_sym();
    point_mutation(&mut pop, bits_per_sym, params.prob_mut, &mut rng);

    let mut num_ones = 0;
    for ind in pop.0 {
        for code_word in ind.0 {
            for bit_index in 0..bits_per_sym {
                if code_word & (1 << bit_index) != 0 {
                    num_ones += 1;
                }
            }
        }
    }

    println!("num ones = {}", num_ones);
    let percent_ones = num_ones as f64 / (num_inds as f64 * num_words as f64 * bits_per_sym as f64);
    println!("percent ones = {}", percent_ones);
    // NOTE this does a statically likely test within a unit test, which may be a bad idea.
    assert!((percent_ones - 0.5).abs() < 0.005, format!("Percent ones was expected to be 0.5, but was {}", percent_ones));
}

