extern crate rand;
extern crate statrs;
#[cfg(test)] extern crate float_cmp;


use std::cmp::max;
use std::ops::Add;
use std::iter::Sum;
use std::collections::HashMap;

use rand::prelude::*;
use rand::distributions::Distribution;

use statrs::distribution::{Uniform, Geometric};

#[cfg(test)] use float_cmp::*;


pub type EvalFunction<B, R> = Fn(&Ind, &mut B, &mut R) -> f64;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ind(pub Vec<u8>);

impl Ind {
    pub fn to_string<B: Clone>(&self, context: &Context<f64, B>) -> String {
        let mut string = "".to_string();

        for code in self.0.iter() {
            let sym = context.decode(*code);
            string.push_str(&sym.name);
            string.push_str(&"");
        }

        string
    }
    
    pub fn eval<A: Clone, B: Clone>(&self, context: &Context<A, B>, b: &mut B) -> A {
        self.eval_with_stack(context, &Vec::new(), b)
    }

    pub fn eval_with_stack<A: Clone, B: Clone>(&self, context: &Context<A, B>, stack: &Vec<A>, b: &mut B) -> A {
        let mut local_stack = stack.clone();
        self.execute_with_stack(context, &mut local_stack, b);
        match local_stack.pop() {
            Some(result) => result,
            None => context.default.clone(),
        }
    }

    pub fn execute<A: Clone, B: Clone>(&self, context: &Context<A, B>, b: &mut B) -> Vec<A> {
        let mut stack = Vec::new();
        self.execute_with_stack(context, &mut stack, b);
        stack
    }

    pub fn execute_with_stack<A: Clone, B: Clone>(&self, context: &Context<A, B>, stack: &mut Vec<A>, b: &mut B) {
        for code in self.0.iter() {
            let sym = context.decode(*code);
            if stack.len() >= sym.arity.num_in {
                (sym.fun)(stack, b);
            }
        }
    }
}

#[test]
fn test_eval_simple_equation() {
    let terminals =
        vec!(zero_sym::<()>(), one_sym(), two_sym());

    let functions = vec!(plus_sym());

    let context: Context<f64, ()> = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    let mut ind_vec = Vec::new();
    ind_vec.push(2); // one
    ind_vec.push(4); // two
    ind_vec.push(1); // plus
    let ind = Ind(ind_vec);
    let result = ind.eval(&context, &mut ());
    assert!(result.approx_eq(&3.0, 2.0 * ::std::f64::EPSILON, 2), format!("result was {}", result))
}

#[derive(Debug)]
pub struct Pop(pub Vec<Ind>);

impl Pop {
    pub fn create<A, R, B>(params: &Params, context: &Context<A, B>, rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
        let mut pop = Vec::with_capacity(params.pop_size);

        let mut rng = rand::thread_rng();

        let bits_needed = context.bits_per_sym();
        assert!(bits_needed <= 8, "This implementation does not currently support multiple byte symbols");
        let bytes_per_sym = ((bits_needed as f64) / 8.0).ceil() as usize;

        let range = 2_u32.pow(bits_needed as u32);

        for _ in 0..params.pop_size {
            let mut ind_vec = Vec::with_capacity(params.ind_size);
            for _ in 0..params.ind_size {
                ind_vec.push(rng.gen_range(0, range) as u8);
            }
            pop.push(Ind(ind_vec));
        }

        Pop(pop)
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Copy)]
pub struct Arity {
    pub num_in:  usize,
    pub num_out: usize,
}

impl Arity {
    pub fn new(num_in: usize, num_out: usize) -> Self {
        Arity { num_in: num_in, num_out: num_out }
    }
}


impl Add for Arity {
    type Output = Arity;

    fn add(self, other: Arity) -> Arity {
        let mut num_in;
        let mut num_out;

        num_in = self.num_in;
        if other.num_in > self.num_out {
            num_in += other.num_in - self.num_out;
        }

        num_out = other.num_out;
        if self.num_out > other.num_in {
            num_out += self.num_out - other.num_in;
        }
        Arity { num_in:  num_in,
                num_out: num_out,
        }
    }
}

#[test]
fn test_arity_simple_cases() {
    let ar1 = Arity::new(3, 2);
    let ar2 = Arity::new(2, 2);
    let ar3 = Arity::new(5, 1);

    assert!(ar1 + ar2 == Arity::new(3, 2), format!("arity was {:?}", ar1 + ar2));
    assert!(ar1 + ar3 == Arity::new(6, 1), format!("arity was {:?}", ar1 + ar3));
    assert!(ar3 + ar1 == Arity::new(7, 2), format!("arity was {:?}", ar1 + ar3));
}

#[derive(Clone)]
pub struct Sym<'a, A: 'static, B: 'static> {
    pub name: &'static str,
    pub arity: Arity,
    pub fun: &'a Fn(&mut Vec<A>, &mut B),
}

#[derive(Clone)]
pub struct Params {
    pub prob_mut: f64,
    pub prob_one_point_crossover: f64,
    pub prob_two_point_crossover: f64,
    pub prob_rotation: f64,

    pub pop_size: usize,
    pub ind_size: usize,

    pub num_gens: usize,
}

impl Default for Params {
    fn default() -> Self {
        Params {
            prob_mut: 0.001,
            prob_one_point_crossover: 0.6,
            prob_two_point_crossover: 0.6,
            prob_rotation: 0.01,
            pop_size: 25,
            ind_size: 100,
            num_gens: 100,
        }
    }
}

#[derive(Clone)]
pub struct Context<'a, A: Clone + 'static, B: Clone + 'static> {
    pub terminals: Vec<Sym<'a, A, B>>,
    pub functions: Vec<Sym<'a, A, B>>,

    pub default: A,
}

impl<'a, A: Clone, B: Clone + 'static> Context<'a, A, B> {
    pub fn num_symbols(&self) -> usize {
        self.terminals.len() + self.functions.len()
    }

    pub fn bits_per_sym(&self) -> usize {
        let syms_to_encode = max(self.terminals.len(), self.functions.len());
        ((syms_to_encode as f64).log2().ceil()) as usize + 1
    }

    pub fn bytes_per_sym(&self) -> usize {
        ((self.bits_per_sym() as f64) / 8.0).ceil() as usize
    }
}

impl<'a, A: Clone, B: Clone + 'static> Context<'a, A, B> {
    pub fn decode(&self, code: u8) -> Sym<'a, A, B> {
        let is_function = (code & 1) == 1;
        let index = (code >> 1) as usize;
        if is_function {
            self.functions[index % self.functions.len()].clone()
        } else {
            self.terminals[index % self.terminals.len()].clone()
        }
    }
}

pub fn zero<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    stack.push(0.0);
}

pub fn one<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    stack.push(1.0);
}

pub fn two<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    stack.push(2.0);
}

pub fn plus<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1 + arg2);
}

pub fn mult<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1 * arg2);
}

pub fn dup<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    let head = stack.pop().unwrap();
    stack.push(head);
    stack.push(head);
}

pub fn swap<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1);
    stack.push(arg2);
}

pub fn drop<B: 'static>(stack: &mut Vec<f64>, b: &mut B) {
    stack.pop().unwrap();
}

pub fn zero_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'static Fn(&mut Vec<f64>, &mut B) = &zero;
    Sym { name: "0", arity: Arity::new(0, 1), fun: f }
}

pub fn one_sym<'a, B: Clone + 'static>() ->  Sym<'a, f64, B> {
    let f: &'static Fn(&mut Vec<f64>, &mut B) = &one;
    Sym { name: "1", arity: Arity::new(0, 1), fun: f  }
}

pub fn two_sym<'a, B: Clone + 'static>() ->  Sym<'a, f64, B> {
    let f: &'static Fn(&mut Vec<f64>, &mut B) = &two;
    Sym { name: "2", arity: Arity::new(0, 1), fun: f  }
}

pub fn plus_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'a Fn(&mut Vec<f64>, &mut B) = &plus;
    Sym { name: "+", arity: Arity::new(2, 1), fun: f }
}

pub fn mult_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'a Fn(&mut Vec<f64>, &mut B) = &mult;
    Sym { name: "*", arity: Arity::new(2, 1), fun: f }
}

pub fn dup_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'a Fn(&mut Vec<f64>, &mut B) = &dup;
    Sym { name: "dup", arity: Arity::new(1, 2), fun: f }
}

pub fn swap_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'a Fn(&mut Vec<f64>, &mut B) = &swap;
    Sym { name: "swap", arity: Arity::new(2, 2), fun: f }
}

pub fn drop_sym<'a, B: Clone + 'static>() -> Sym<'a, f64, B> {
    let f: &'a Fn(&mut Vec<f64>, &mut B) = &drop;
    Sym { name: "drop", arity: Arity::new(1, 0), fun: f }
}

pub fn symbol_sym<'a>(sym: String) -> Sym<'a, f64, HashMap<String, f64>> {
    let f: &'a Fn(&mut Vec<f64>, &mut HashMap<String, f64>) =
        &move |stack: &mut Vec<f64>, map: &mut HashMap<String, f64>| {
            stack.push(*map.get(&sym).unwrap());
    };
    Sym { name: "drop", arity: Arity::new(0, 1), fun: f }
}

pub fn point_mutation_naive<R: Rng>(pop: &mut Pop, bits_used: usize, pm: f64, rng: &mut R) {
    let sampler = Uniform::new(0.0, 1.0).unwrap();

    for ind in pop.0.iter_mut() {
        point_mutate_naive(ind, bits_used, pm, rng);
    }
}

pub fn point_mutate_naive<R: Rng>(ind: &mut Ind, bits_used: usize, pm: f64, rng: &mut R) {
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
    let ind_len_bits = pop.0[0].0.len() * bits_used;

    let sampler = Geometric::new(pm).unwrap();

    for ind in pop.0.iter_mut() {
        point_mutate(ind, bits_used, pm, rng);
    }
}

pub fn point_mutate<R: Rng>(ind: &mut Ind, bits_used: usize, pm: f64, rng: &mut R) {
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

#[test]
fn test_point_mutation_flips_bits() {
    let terminals =
        vec!(zero_sym::<()>(), one_sym::<()>(), two_sym::<()>());

    let functions =
        vec!(plus_sym::<()>());

    let num_inds  = 200;
    let num_words = 200;

    let params: Params = Params {
        prob_mut: 0.001,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.9,
        pop_size: num_inds,
        ind_size: num_words,
        num_gens: 100,
    };

    let context = Context {
        terminals: terminals,
        functions: functions,
        default: 0.0,
    };

    let mut rng = thread_rng();

    let mut pop = Pop::create(&params, &context, &mut rng);
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

pub fn crossover_one_point<R: Rng>(pop: &mut Pop, words_per_ind: usize, bits_per_sym: usize, pc1: f64, rng: &mut R) {
    let pc1_sampler = Uniform::new(0.0, 1.0).unwrap();
    let cross_point_sampler = Uniform::new(0.0, (words_per_ind * bits_per_sym) as f64).unwrap();

    for mut pair in pop.0.chunks_mut(2) {
        if pair.len() != 2 {
            break;
        }

        if pc1_sampler.sample(rng) < pc1 {
            let cross_point = cross_point_sampler.sample(rng) as usize;

            cross_at_point(&mut pair, bits_per_sym, cross_point);
        }
    }
}

pub fn cross_at_point(pair: &mut [Ind], bits_per_sym: usize, cross_point: usize) {
    let cross_word_index = cross_point / bits_per_sym;
    for word_index in 0..cross_word_index {
        let tmp = pair[0].0[word_index];
        pair[0].0[word_index] = pair[1].0[word_index];
        pair[1].0[word_index] = tmp;
    }
    // cross the word that the cross point is within
    let l1 = pair[0].0[cross_word_index];
    let l2 = pair[1].0[cross_word_index];
    let bit_mask = (2_u32.pow((cross_point % bits_per_sym) as u32) - 1) as u8;
    pair[0].0[cross_word_index] = (l1 & bit_mask) | (l2 & !bit_mask);
    pair[1].0[cross_word_index] = (l2 & bit_mask) | (l1 & !bit_mask);
}

#[test]
fn test_cross_at_point() {
    let ind1 = Ind(vec!(0xF, 0xF, 0xF, 0xF, 0xF));
    let ind2 = Ind(vec!(0x0, 0x0, 0x0, 0x0, 0x0));
    let mut pair = [ind1, ind2];

    cross_at_point(&mut pair, 4, 10);
    assert!(pair[0] == Ind(vec!(0, 0, 3, 0xF, 0xF)));
    assert!(pair[1] == Ind(vec!(0xF, 0xF, 0xC, 0x0, 0x0)));
}

pub fn crossover_two_point<R: Rng>(pop: &mut Pop, words_per_ind: usize, bits_per_sym: usize, pc2: f64, rng: &mut R) {
    let ind_len = pop.0[0].0.len();

    let pc2_sampler = Uniform::new(0.0, 1.0).unwrap();
    let cross_point_sampler = Uniform::new(0.0, (words_per_ind * bits_per_sym) as f64).unwrap();

    for mut pair in pop.0.chunks_mut(2) {
        if pair.len() != 2 {
            break;
        }

        if pc2_sampler.sample(rng) < pc2 {
            let cross_point_one = cross_point_sampler.sample(rng) as usize;
            let cross_point_two = cross_point_sampler.sample(rng) as usize;

            let mut locs = [cross_point_one, cross_point_two];
            locs.sort();
            cross_at_points(&mut pair, bits_per_sym, &locs);
        }
    }
}

// Generic multipoint crossover. This version skips indices that will not be effected,
// making it somewhat more complex then necessary.
pub fn cross_at_points(pair: &mut [Ind], bits_per_sym: usize, cross_points: &[usize]) {
    let ind_len = pair[0].0.len();

    let mut bounded_cross_points = Vec::new();

    // add boundary indices to cross points
    bounded_cross_points.push(0);
    bounded_cross_points.extend_from_slice(cross_points);
    bounded_cross_points.push(ind_len * bits_per_sym - 1);

    // this is used to flip between where we want to do bitwise
    // crossover- the start or end index of the cross points
    let mut flip_flop = true;

    // for each pair of indices
    for point_pair in bounded_cross_points.chunks(2) {
        if point_pair.len() != 2 {
            break;
        }
        // get the word indices to start and end the crossover
        let cross_start = point_pair[0] / bits_per_sym;
        let mut cross_end   = point_pair[1] / bits_per_sym;

        // set up our alternating positions
        let mut first_side = 0;
        let mut other_side = 1;
        let mut cross_index = cross_start;
        if flip_flop {
            first_side = 1;
            other_side = 0;
            cross_index = cross_end;
        }

        // if crossing the end of a word, we may need to go
        // off by 1 to get the right crossed indices
        if point_pair[1] % bits_per_sym != 0 {
            cross_end += other_side;
        }

        // for each index, swap words
        for index in cross_start..cross_end {
            let tmp = pair[0].0[index];
            pair[0].0[index] = pair[1].0[index];
            pair[1].0[index] = tmp;
        }

        // cross the word that the cross point is within
        let cross_bit_index = point_pair[first_side] % bits_per_sym;
        if cross_bit_index != (bits_per_sym - 1) {
            let (first, second) = cross_word(pair[first_side].0[cross_index],
                                             pair[other_side].0[cross_index],
                                             cross_bit_index as u8);

            pair[0].0[cross_index] = second;
            pair[1].0[cross_index] = first;
        }

        flip_flop = !flip_flop;
    }
}

#[test]
fn test_cross_at_points() {
    let mut ind1 = Ind(vec!(0x00, 0x00, 0x00, 0x00, 0x00));
    let mut ind2 = Ind(vec!(0x0F, 0x0F, 0x0F, 0x0F, 0x0F));

    let pair = &mut [ind1, ind2];

    cross_at_points(pair, 4, &[1, 6]);
    assert!(pair[0] == Ind(vec!(0x01, 0x03, 0x0F, 0x0F, 0x0F)));
    assert!(pair[1] == Ind(vec!(0x0E, 0x0C, 0x00, 0x00, 0x00)));
}

pub fn cross_word(first: u8, second: u8, bit_index: u8) -> (u8, u8) {
    let bit_mask = (2_u32.pow(bit_index as u32) - 1) as u8;

    let first_result  = (first  & !bit_mask) | (second & bit_mask);
    let second_result = (second & !bit_mask) | (first  & bit_mask);

    (first_result, second_result)
}

#[test]
fn test_cross_word() {
    let (first, second) = cross_word(0xff, 0x00, 4);
    assert!(first  == 0xF0, format!("was {:b}, expected {:b}", first,  0xF0));
    assert!(second == 0x0F, format!("was {:b}, expected {:b}", second, 0x0F));
}

pub fn cross_at_points_naive(pair: &mut [Ind], bits_per_sym: usize, cross_points: &[usize]) {
    let ind_len = pair[0].0.len();

    let mut cross_point_index = 0;

    for index in 0..ind_len {
        let tmp = pair[0].0[index];
        pair[0].0[index] = pair[1].0[index];
        pair[1].0[index] = tmp;

        // are there more cross points?
        if cross_points.len() > cross_point_index {
            // time to move to next cross point?
            let cross_index = cross_points[cross_point_index] / bits_per_sym;
            if index == cross_index {
                let cross_bit_index = cross_points[cross_point_index] % bits_per_sym;
                if cross_bit_index != (bits_per_sym - 1) {
                    let (first, second) = cross_word(pair[0].0[index],
                                                     pair[1].0[index],
                                                     cross_bit_index as u8);

                    pair[0].0[index] = second;
                    pair[1].0[index] = first;
                }
                pair.swap(0, 1);
                cross_point_index += 1;
            }
        }
    }
}

#[test]
fn test_cross_at_points_naive() {
    let ind1 = Ind(vec!(0x00, 0x00, 0x00, 0x00, 0x00));
    let ind2 = Ind(vec!(0x0F, 0x0F, 0x0F, 0x0F, 0x0F));

    let pair = &mut [ind1, ind2];

    cross_at_points_naive(pair, 4, &[1, 6]);
    assert!(pair[0] == Ind(vec!(0x01, 0x03, 0x0F, 0x0F, 0x0F)));
    assert!(pair[1] == Ind(vec!(0x0E, 0x0C, 0x00, 0x00, 0x00)));
}

pub fn rotation<R: Rng>(pop: &mut Pop, pr: f64, rng: &mut R) {
    let ind_len = pop.0[0].0.len();

    let rotation_sampler = Uniform::new(0.0, 1.0).unwrap();
    let rotation_point_sampler = Uniform::new(0.0, ind_len as f64).unwrap();

    for ind in pop.0.iter_mut() {
        if rotation_sampler.sample(rng) < pr {
            let rotation_point = rotation_point_sampler.sample(rng) as usize;

        }
    }
}

pub fn rotate(ind: &mut Ind, rotation_point: usize) {
    let ind_len = ind.0.len();

    let mut index = 0;
    let mut tmp = ind.0[0];

    let mut tmp = ind.0[0];
    for _ in 0..ind.0.len() {
        let other_index = (index + rotation_point) % ind_len; 

        let tmp2 = ind.0[other_index];
        ind.0[other_index] = tmp;
        tmp = tmp2;

        index = other_index;
    }
}

#[test]
fn test_rotate() {
    let mut ind = Ind(vec!(0, 1, 2, 3, 4));

    let rotation_point = 3;

    rotate(&mut ind, rotation_point);

    let expected = Ind(vec!(2, 3, 4, 0, 1));

    assert!(ind == expected, format!("{:?} != {:?}", ind, expected));
}

pub fn stochastic_universal_sampling<R: Rng>(pop: &Pop, fitnesses: Vec<f64>, rng: &mut R) -> Pop {
    let increment = fitnesses.iter().sum::<f64>() / fitnesses.len() as f64;
    let offset = Uniform::new(0.0, increment).unwrap().sample(rng);

    select_stochastic_universal(pop, fitnesses, offset, increment)
}

pub fn select_stochastic_universal(pop: &Pop, fitnesses: Vec<f64>, offset: f64, increment: f64) -> Pop {
    let num_inds = pop.0.len();
    let mut new_pop = Vec::with_capacity(num_inds);

    let mut offset = offset;
    let mut accum_fitness = 0.0;
    let mut num_selections = 0;
    let mut ind_index = 0;

    while num_selections < pop.0.len() {
        accum_fitness += fitnesses[ind_index];

        while offset <= accum_fitness {
            new_pop.push(Ind(pop.0[ind_index].0.clone()));
            offset += increment;
            num_selections += 1;
        }
        ind_index += 1;
    }

    Pop(new_pop)
}

pub fn evaluate<R, A, B>(pop: &Pop,
                     context: &Context<A, B>,
                     state: &B,
                     eval_ind: &EvalFunction<B, R>,
                     rng: &mut R) -> Vec<f64>
    where R: Rng,
          A: Clone,
          B: Clone {
    let mut fitnesses = Vec::new();

    for ind in pop.0.iter() {
        let mut local_state = state.clone();
        //let f: F = |context: &Context<A, B>| -> A { ind.eval(context, &mut state) };
        let fitness = eval_ind(ind, &mut local_state, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

pub fn rgep<R: Rng, B: Clone>(params: &Params,
                          context: &Context<f64, B>,
                          state: &B,
                          eval_ind: &EvalFunction<B, R>,
                          rng: &mut R) -> Pop {
    let mut pop = Pop::create(&params, &context, rng);

    let bits_per_sym = context.bits_per_sym();

    for num_gen in 0..params.num_gens {
        rotation(&mut pop, params.prob_rotation, rng);
        point_mutation(&mut pop, bits_per_sym, params.prob_mut, rng);
        crossover_one_point(&mut pop, params.ind_size, bits_per_sym, params.prob_one_point_crossover, rng);
        crossover_two_point(&mut pop, params.ind_size, bits_per_sym, params.prob_two_point_crossover, rng);
        let fitnesses = evaluate(&pop, context, state, eval_ind, rng);
        pop = stochastic_universal_sampling(&pop, fitnesses, rng);
    }

    pop
}

