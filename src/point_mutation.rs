use rand::prelude::*;
use std::iter::IntoIterator;

use num::PrimInt;

use statrs::distribution::{Uniform, Geometric};

use im::vector::Vector;

use types::*;

#[cfg(test)]
use crate::rgep::*;

#[cfg(test)]
use crate::ga::*;


pub fn point_mutation_naive<T: PrimInt, R: Rng>(pop: &mut Pop<T>, bits_used: usize, pm: f64, rng: &mut R) {
    for ind in pop.0.iter_mut() {
        point_mutate_naive(ind.0.iter_mut(), bits_used, pm, rng);
    }
}

pub fn point_mutate_naive<'a, I, T, R: Rng>(ind: I, bits_used: usize, pm: f64, rng: &mut R) 
    where R: Rng,
          I: 'a + IntoIterator<Item=&'a mut T>,
          T: PrimInt + 'a {
    let sampler = Uniform::new(0.0, 1.0).unwrap();

    for loc in ind.into_iter() {
        for bit_index in 0..bits_used {
            if sampler.sample(rng) < pm {
                *loc = *loc ^ (num::one::<T>() << bit_index);
            }
        }
    }
}

pub fn point_mutation<T: PrimInt, R: Rng>(pop: &mut Pop<T>, bits_used: usize, pm: f64, rng: &mut R) {
    for ind in pop.0.iter_mut() {
        point_mutate(ind, bits_used, pm, rng);
    }
}

pub fn point_mutate<T, R>(ind: &mut Ind<T>, bits_used: usize, pm: f64, rng: &mut R) 
    where R: Rng,
          T: PrimInt {
    let ind_len_bits = ind.0.len() * bits_used;

    let sampler = Geometric::new(pm).unwrap();

    let mut next_loc_bits = sampler.sample(rng) as usize;
    
    while next_loc_bits < ind_len_bits {
        let next_loc = next_loc_bits / bits_used;
        let bit_index = next_loc_bits % bits_used;

        let word = ind.0[next_loc];

        ind.0[next_loc] = word ^ (T::one() << bit_index);

        next_loc_bits += sampler.sample(rng) as usize;
    }
}

pub fn point_mutate_im<T, R>(ind: &mut Vector<T>, bits_used: usize, pm: f64, rng: &mut R) 
    where R: Rng,
          T: PrimInt {
    let ind_len_bits = ind.len() * bits_used;

    let sampler = Geometric::new(pm).unwrap();

    let mut next_loc_bits = sampler.sample(rng) as usize;
    
    while next_loc_bits < ind_len_bits {
        let next_loc = next_loc_bits / bits_used;
        let bit_index = next_loc_bits % bits_used;

        let word = ind.get_mut(next_loc).unwrap();

        *word = *word ^ (T::one() << bit_index);

        next_loc_bits += sampler.sample(rng) as usize;
    }
}

