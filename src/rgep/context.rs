use std::cmp::max;

use crate::rgep::program::*;
use crate::types::*;


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
    pub fn decode(&self, code: u8) -> &'a Sym<'a, A, B> {
        let is_function = (code & 1) == 1;
        let index = (code >> 1) as usize;
        if is_function {
            return &'a self.functions[index % self.functions.len()];
        } else {
            return &'a self.terminals[index % self.terminals.len()];
        }
    }
}

impl<'a, A: Clone, B: Clone + 'static> Context<'a, A, B> {
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

    pub fn compile(&self, ind: &Ind<u8>) -> Program<'a, A, B> {
        let mut program = Program(Vec::with_capacity(ind.0.len()));

        self.compile_to(ind, &mut program);

        program
    }

    pub fn compile_to(&self, ind: &Ind<u8>, prog: &mut Program<'a, A, B>) {
        prog.0.clear();
        for code in ind.0.iter() {
            let sym = self.decode(*code);
            prog.0.push(sym);
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

