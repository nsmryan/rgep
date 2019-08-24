use std::collections::HashMap;
use std::iter;
use std::iter::*;
use std::cmp::*;
use std::ops::Add;
use std::rc::Rc;

#[cfg(test)]
use float_cmp::*;

use rand::prelude::*;

use crate::types::*;
use crate::crossover::*;
use crate::point_mutation::*;
use crate::rotation::*;
use crate::selection::*;


pub struct Context<A: Clone + 'static, B: Clone + 'static> {
    pub terminals: Vec<Sym<A, B>>,
    pub functions: Vec<Sym<A, B>>,

    pub default: A,
}

impl<A: Clone, B: Clone + 'static> Context<A, B> {
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

impl<A: Clone, B: Clone + 'static> Context<A, B> {
    pub fn decode(&self, code: u8) -> &Sym<A, B> {
        let is_function = (code & 1) == 1;
        let index = (code >> 1) as usize;
        if is_function {
            &self.functions[index % self.functions.len()]
        } else {
            &self.terminals[index % self.terminals.len()]
        }
    }
}

impl<A: Clone, B: Clone + 'static> Context<A, B> {
    pub fn to_string(&self, ind: &Ind<u8>) -> String {
        let mut string = "".to_string();

        for code in ind.0.iter() {
            let sym = self.decode(*code);
            string.push_str(&sym.name);
            string.push_str(&"");
        }

        string
    }
    
    pub fn eval(&self, ind: &Ind<u8>, b: &mut B) -> A {
        self.eval_with_stack(ind, &Vec::new(), b)
    }

    pub fn eval_with_stack(&self, ind: &Ind<u8>, stack: &Vec<A>, b: &mut B) -> A {
        let mut local_stack = stack.clone();
        self.exec_with_stack(ind, &mut local_stack, b);
        match local_stack.pop() {
            Some(result) => result,
            None => self.default.clone(),
        }
    }

    pub fn exec(&self, ind: &Ind<u8>, b: &mut B) -> Vec<A> {
        let mut stack = Vec::new();
        self.exec_with_stack(ind, &mut stack, b);
        stack
    }

    pub fn exec_with_stack(&self, ind: &Ind<u8>, stack: &mut Vec<A>, b: &mut B) {
        for code in ind.0.iter() {
            let sym = self.decode(*code);
            if stack.len() >= sym.arity.num_in {
                (sym.fun)(stack, b);
            }
        }
    }

    pub fn compile(&self, ind: &Ind<u8>) -> Program<A, B> {
        let mut program = Program(Vec::with_capacity(ind.0.len()));

        self.compile_to(ind, &mut program);

        program
    }

    pub fn compile_to(&self, ind: &Ind<u8>, prog: &mut Program<A, B>) {
        prog.0.clear();
        for code in ind.0.iter() {
            let sym = self.decode(*code);
            prog.0.push(sym.clone());
        }
    }
}

#[test]
fn test_eval_simple_equation() {
    let terminals =
        vec!(zero_sym(), one_sym(), two_sym());

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
    let result = context.eval(&ind, &mut ());
    assert!(result.approx_eq(&3.0, 2.0 * ::std::f64::EPSILON, 2), format!("result was {}", result))
}

pub type EvalFunction<A, B, R> = Fn(&Program<A, B>, &mut B, &mut R) -> f64;

pub type Variables<A> = HashMap<String, A>;

pub struct Program<A, B>(pub Vec<Sym<A, B>>);

impl<A, B> Program<A, B> {
    pub fn eval(&self, state: &mut B, default: A) -> A {
        let mut stack = Vec::new();
        self.eval_with_stack(state, default, &mut stack)
    }

    pub fn eval_with_stack(&self, state: &mut B, default: A, stack: &mut Vec<A>) -> A {
        self.exec_with_stack(state, stack);
        if stack.len() > 0 {
            stack.pop().unwrap()
        } else {
            default
        }
    }

    pub fn exec(&self, state: &mut B) -> Vec<A> {
        let mut stack = Vec::new();
        self.exec_with_stack(state, &mut stack);
        stack
    }

    pub fn exec_with_stack(&self, state: &mut B, stack: &mut Vec<A>) {
        for sym in self.0.iter() {
            if stack.len() >= sym.arity.num_in {
                (sym.fun)(stack, state);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = "".to_string();

        for sym in self.0.iter() {
            string.push_str(&sym.name);
            string.push_str(&"");
        }

        string
    }
}

#[derive(Clone)]
pub enum Node<A, B> {
    Node(Sym<A, B>, Vec<Node<A, B>>),
    Leaf(Sym<A, B>)
}

impl<A: Clone, B: Clone> Node<A, B> {
    pub fn linearize(&self) -> Vec<Sym<A, B>> {
        let mut syms = Vec::new();

        self.linearize_helper(&mut syms);

        syms
    }

    pub fn linearize_helper(&self, syms: &mut Vec<Sym<A, B>>) {
        match self {
            Node::Leaf(sym) => {
                syms.push(sym.clone());
            },

            Node::Node(sym, children) => {
                for node in children.iter().rev() {
                    node.linearize_helper(syms);
                }
                syms.push(sym.clone());
            },
        }
    }

    pub fn eval(&self, state: &mut B) -> A {
        let mut stack = Vec::new();

        match self {
            Node::Leaf(sym) => {
                assert!(sym.arity.num_in == 0);
                assert!(sym.arity.num_out == 1);
                (sym.fun)(&mut stack, state);
                stack.pop().unwrap()
            },

            Node::Node(sym, children) => {
                for child in children {
                    stack.push(child.eval(state));
                }

                (sym.fun)(&mut stack, state);

                stack.pop().unwrap()
            },
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            _ => false,
        }
    }

    pub fn sym(&self) -> Sym<A, B> {
        match self {
            Node::Leaf(sym) => sym.clone(),
            Node::Node(sym, _) => sym.clone(),
        }
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


pub struct Sym<A, B> {
    pub name: String,
    pub arity: Arity,
    pub fun: Rc<Fn(&mut Vec<A>, &mut B)>,
}

impl<A: Clone, B: Clone> Clone for Sym<A, B> {
    fn clone(&self) -> Self {
        Sym { name: self.name.clone(),
              arity: self.arity,
              fun: self.fun.clone(),
        }
    }
}

impl<A, B> Sym<A, B> {
    pub fn new(name: String, arity: Arity, fun: Rc<Fn(&mut Vec<A>, &mut B)>) -> Sym<A, B> {
        Sym { name: name,
              arity: arity,
              fun: fun,
        }
    }
}


#[derive(Clone)]
pub struct RgepParams {
    pub prob_mut: f64,
    pub prob_one_point_crossover: f64,
    pub prob_two_point_crossover: f64,
    pub prob_rotation: f64,

    pub pop_size: usize,
    pub ind_size: usize,

    pub elitism: usize,

    pub num_gens: usize,
}

impl Default for RgepParams {
    fn default() -> Self {
        RgepParams {
            prob_mut: 0.001,
            prob_one_point_crossover: 0.6,
            prob_two_point_crossover: 0.6,
            prob_rotation: 0.01,
            pop_size: 25,
            ind_size: 100,
            elitism: 1,
            num_gens: 100,
        }
    }
}

pub fn create_rgep<A, R, B>(params: &RgepParams, context: &Context<A, B>, rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
    let mut pop = Vec::with_capacity(params.pop_size);

    let bits_needed = context.bits_per_sym();
    assert!(bits_needed <= 8, "This implementation does not currently support multiple byte symbols");

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

pub fn create_rgep_fast(params: &RgepParams) -> Pop {
    let ind = Ind(iter::repeat(0x0).take(params.ind_size).collect());
    Pop(iter::repeat(ind).take(params.pop_size).collect())
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
        context.compile_to(&ind, &mut prog);
        let fitness = eval_prog(&prog, &mut local_state, rng);
        fitnesses.push(fitness);
    }

    fitnesses
}

pub fn rgep<R, A, B>(params: &RgepParams,
                     context: &Context<A, B>,
                     state: &B,
                     eval_ind: &EvalFunction<A, B, R>,
                     rng: &mut R) -> Pop 
    where R: Rng, A: Clone, B: Clone {
    let mut pop = Box::new(create_rgep(&params, &context, rng));
    let mut alt_pop = Box::new(create_rgep_fast(&params));

    let bits_per_sym = context.bits_per_sym();

    for _ in 0..params.num_gens {
        rotation(&mut pop, params.prob_rotation, rng);
        point_mutation(&mut pop, bits_per_sym, params.prob_mut, rng);
        crossover_one_point(&mut pop, params.ind_size, bits_per_sym, params.prob_one_point_crossover, rng);
        crossover_two_point(&mut pop, params.ind_size, bits_per_sym, params.prob_two_point_crossover, rng);
        let fitnesses = rgep_evaluate(&pop, context, state, eval_ind, rng);
        stochastic_universal_sampling(&pop, &mut alt_pop, fitnesses, params.elitism, rng);

        std::mem::swap(&mut pop, &mut alt_pop);
    }

    *pop
}

