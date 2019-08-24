use std::rc::Rc;
use std::ops::{Add};
use std::cmp::max;

use crate::types::*;


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
