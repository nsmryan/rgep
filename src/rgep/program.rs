use std::rc::Rc;
use std::ops::{Add};


/// Stack program arity, like a forth call stack
/// signature.
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

/// A symbol in a stack program
pub struct Sym<A, B> {
    pub name: String,
    pub arity: Arity,
    pub fun: Fn(&mut Vec<A>, &mut B),
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
    pub fn new(name: String, arity: Arity, fun: Fn(&mut Vec<A>, &mut B)) -> Sym<A, B> {
        Sym { name: name,
              arity: arity,
              fun: fun,
        }
    }
}


/// A stack program as a sequence of symbols
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

