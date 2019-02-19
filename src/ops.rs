use std::rc::Rc;
use std::boxed::Box;
use std::ops::{Add, Sub, Mul, Div, Rem};
use std::num::FromPrimitive;
use std::fmt::Display;

use types::*;


#[derive(Clone, PartialEq)]
pub enum Arith {
    Add(Box<Arith>, Box<Arith>),
    Sub(Box<Arith>, Box<Arith>),
    Mult(Box<Arith>, Box<Arith>),
    Div(Box<Arith>, Box<Arith>),
    Const(f64),
    Var(String),
}

impl Arith {
    pub fn eval(&self, context: &Variables) -> f64 {
        match self {
            Arith::Add(exp1, exp2) => {
                exp1.eval(context) + exp2.eval(context)
            },

            Arith::Sub(exp1, exp2) => {
                exp1.eval(context) - exp2.eval(context)
            },

            Arith::Mult(exp1, exp2) => {
                exp1.eval(context) * exp2.eval(context)
            },

            Arith::Div(exp1, exp2) => {
                let denom = exp2.eval(context);
                if denom == 0.0 {
                    0.0
                } else {
                    exp1.eval(context) / denom
                }
            },

            Arith::Const(constant) => {
                *constant
            },

            Arith::Var(name) => {
                *context.get(name).unwrap()
            },

        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Arith::Const(_) => true,
            Arith::Var(_) => true,
            _ => false,
        }
    }

    pub fn is_node(&self) -> bool {
        !self.is_leaf()
    }

    pub fn is_const(&self) -> bool {
        match self {
            Arith::Const(_) => true,
            _ => false,
        }
    }

    pub fn is_constant(&self, test_contant: f64) -> bool {
        match self {
            Arith::Const(constant) => *constant == test_contant,
            _ => false,
        }
    }

    pub fn simplify(&self) -> Arith {
        match self.clone() {
            Arith::Add(exp1, exp2) => {
                let exp1 = exp1.simplify();
                let exp2 = exp2.simplify();

                if exp1.is_const() && exp2.is_const() {
                    match (exp1, exp2) {
                        (Arith::Const(c1), Arith::Const(c2)) => Arith::Const(c1 + c2),
                        _ => panic!("Simplify sum should not have reached this code!"),
                    }
                } else if exp1.is_constant(0.0) {
                    exp2
                } else if exp2.is_constant(0.0) {
                    exp1
                } else if exp2.is_const() {
                    Arith::Add(Box::new(exp2), Box::new(exp1))
                } else {
                    Arith::Add(Box::new(exp1), Box::new(exp2))
                }
            },

            Arith::Sub(exp1, exp2) => {
                let exp1 = exp1.simplify();
                let exp2 = exp2.simplify();

                if exp1.is_const() && exp2.is_const() {
                    match (exp1, exp2) {
                        (Arith::Const(c1), Arith::Const(c2)) => Arith::Const(c1 - c2),
                        _ => panic!("Simplify sub should not have reached this code!"),
                    }
                } else {
                    Arith::Sub(Box::new(exp1), Box::new(exp2))
                }
            },

            Arith::Mult(exp1, exp2) => {
                let exp1 = exp1.simplify();
                let exp2 = exp2.simplify();

                if exp1.is_const() && exp2.is_const() {
                    match (exp1, exp2) {
                        (Arith::Const(c1), Arith::Const(c2)) => Arith::Const(c1 * c2),
                        _ => panic!("Simplify mult should not have reached this code!"),
                    }
                } else if exp1.is_constant(0.0) || exp2.is_constant(0.0) {
                    Arith::Const(0.0)
                } else if exp1.is_constant(1.0) {
                    exp2
                } else if exp2.is_constant(1.0) {
                    exp1
                } else if exp2.is_const() {
                    Arith::Mult(Box::new(exp2), Box::new(exp1.simplify()))
                } else {
                    Arith::Mult(Box::new(exp1), Box::new(exp2))
                }
            },

            Arith::Div(exp1, exp2) => {
                let exp1 = exp1.simplify();
                let exp2 = exp2.simplify();

                if exp2.is_constant(0.0) {
                    Arith::Const(1.0 / 0.0)
                } else if exp1.is_const() && exp2.is_const() {
                    match (exp1, exp2) {
                        (Arith::Const(c1), Arith::Const(c2)) => Arith::Const(c1 / c2),
                        _ => panic!("Simplify div should not have reached this code!"),
                    }
                } else {
                    Arith::Div(Box::new(exp1), Box::new(exp2))
                }
            },

            Arith::Const(constant) => {
                Arith::Const(constant)
            },

            Arith::Var(name) => {
                Arith::Var(name.clone())
            },

        }
    }

    pub fn to_string_infix(&self) -> String {
        let mut string = "".to_string();
        match self {
            Arith::Add(exp1, exp2) => {
                string.push_str(&"(".to_string());
                string.push_str(&exp1.to_string_infix());
                string.push_str(&"+".to_string());
                string.push_str(&exp2.to_string_infix());
                string.push_str(&")".to_string());
            },

            Arith::Sub(exp1, exp2) => {
                string.push_str(&"(".to_string());
                string.push_str(&exp1.to_string_infix());
                string.push_str(&"-".to_string());
                string.push_str(&exp2.to_string_infix());
                string.push_str(&")".to_string());
            },

            Arith::Mult(exp1, exp2) => {
                string.push_str(&"(".to_string());
                string.push_str(&exp1.to_string_infix());
                string.push_str(&"*".to_string());
                string.push_str(&exp2.to_string_infix());
                string.push_str(&")".to_string());
            },

            Arith::Div(exp1, exp2) => {
                string.push_str(&"(".to_string());
                string.push_str(&exp1.to_string_infix());
                string.push_str(&"/".to_string());
                string.push_str(&exp2.to_string_infix());
                string.push_str(&")".to_string());
            },

            Arith::Const(constant) => {
                string.push_str(&constant.to_string());
            },

            Arith::Var(name) => {
                string.push_str(&name.to_string());
            },
        }

        string
    }
}

pub fn add_expr() -> Sym<Arith, Variables> {
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Add(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "+".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn sub_expr() -> Sym<Arith, Variables> {
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Sub(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "-".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn div_expr() -> Sym<Arith, Variables> {
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Div(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "/".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn mult_expr() -> Sym<Arith, Variables> {
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Mult(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "*".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn const_expr(constant: f64) -> Sym<Arith, Variables> {
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            stack.push(Arith::Const(constant));
    });
    Sym { name: constant.to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn var_expr(name: String) -> Sym<Arith, Variables> {
    let sym_name = name.clone();
    let f: Rc<Fn(&mut Vec<Arith>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<Arith>, _map: &mut Variables| {
            stack.push(Arith::Var(name.clone()));
    });
    Sym { name: sym_name, arity: Arity::new(0, 1), fun: f }
}

pub fn dup<A: Clone, B>(stack: &mut Vec<A>, _b: &mut B) {
    let head = stack.pop().unwrap();
    stack.push(head.clone());
    stack.push(head.clone());
}

pub fn swap<A, B>(stack: &mut Vec<A>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1);
    stack.push(arg2);
}

pub fn drop<A, B>(stack: &mut Vec<A>, _b: &mut B) {
    stack.pop().unwrap();
}

pub fn rot<A, B>(stack: &mut Vec<A>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    let arg3 = stack.pop().unwrap();
    stack.push(arg1);
    stack.push(arg3);
    stack.push(arg2);
}

pub fn nip<A, B>(stack: &mut Vec<A>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let _arg2 = stack.pop().unwrap();
    stack.push(arg1);
}

pub fn tuck<A: Clone, B>(stack: &mut Vec<A>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1.clone());
    stack.push(arg2);
    stack.push(arg1);
}

pub fn make_const<A: 'static + ToString + Copy, B: 'static>(constant: A) -> Sym<A, B> {
    let f: Rc<Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        stack.push(constant);
    });
    Sym::new(constant.to_string(), Arity::new(0, 1), f)
}

pub fn make_binary<A, B>(name: &str, f: Rc<Fn(A, A) -> A>) -> Sym<A, B>
    where A: 'static + ToString + Copy, B: 'static {
    let f: Rc<Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        let arg1 = stack.pop().unwrap();
        let arg2 = stack.pop().unwrap();
        stack.push(f(arg1, arg2));
    });
    Sym::new(name.to_string(), Arity::new(2, 1), f)
}

pub fn make_unary<A, B>(name: &str, f: Rc<Fn(A) -> A>) -> Sym<A, B>
    where A: 'static + ToString + Copy, B: 'static {
    let f: Rc<Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        let arg = stack.pop().unwrap();
        stack.push(f(arg));
    });
    Sym::new(name.to_string(), Arity::new(1, 1), f)
}

pub fn zero_sym<B:'static>() -> Sym<f64, B> {
    make_const(0.0)
}

pub fn one_sym<B:'static>() ->  Sym<f64, B> {
    make_const(1.0)
}

pub fn two_sym<B:'static>() ->  Sym<f64, B> {
    make_const(2.0)
}

pub fn plus_sym<A, B>() -> Sym<A, B> 
    where A: Add<Output=A> + Display + Copy + 'static, B:'static {
    make_binary("+", Rc::new(|a, b| a + b))
}

pub fn sub_sym<A, B>() -> Sym<A, B>
    where A: Sub<Output=A> + Display + Copy + 'static, B:'static {
    make_binary("-", Rc::new(|a, b| a - b))
}

pub fn mult_sym<A, B>() -> Sym<A, B>
    where A: Mul<Output=A> + Display + Copy + 'static, B:'static {
    make_binary("*", Rc::new(|a, b| a * b))
}

pub fn mod_sym<A, B>() -> Sym<A, B>
    where A: Rem<Output=A> + Display + Copy + 'static, B:'static {
    make_binary("%", Rc::new(|a, b| a % b))
}

pub fn div_sym<A, B>() -> Sym<A, B>
    where A: Div<Output=A> + Zero + Display + Copy + 'static, B:'static {
    make_binary("/", Rc::new(|a, b| {
        if b == A::zero() {
            0.0
        } else {
            a / b
        }
    }))
}

pub fn and_sym<B:'static>() -> Sym<u32, B> {
    make_binary("&", Rc::new(|a, b| a & b))
}

pub fn or_sym<B:'static>() -> Sym<u32, B> {
    make_binary("|", Rc::new(|a, b| a | b))
}

pub fn xor_sym<B:'static>() -> Sym<u32, B> {
    make_binary("x", Rc::new(|a, b| a ^ b))
}

pub fn not_sym<B:'static>() -> Sym<u32, B> {
    make_unary("-", Rc::new(|a| !a))
}

pub fn dup_sym<A: 'static + Clone, B: 'static>() -> Sym<A, B> {
    Sym::new("dup".to_string(), Arity::new(1, 2), Rc::new(dup))
}

pub fn swap_sym<A: 'static, B: 'static>() -> Sym<A, B> {
    Sym::new("swap".to_string(), Arity::new(2, 2), Rc::new(swap))
}

pub fn drop_sym<A: 'static, B: 'static>() -> Sym<A, B> {
    Sym::new("drop".to_string(), Arity::new(1, 0), Rc::new(drop))
}

pub fn nip_sym<A: 'static, B: 'static>() -> Sym<A, B> {
    Sym::new("drop".to_string(), Arity::new(2, 1), Rc::new(nip))
}

pub fn tuck_sym<A: 'static + Clone, B: 'static>() -> Sym<A, B> {
    Sym::new("tuck".to_string(), Arity::new(2, 3), Rc::new(tuck))
}

pub fn symbol_sym(sym: String) -> Sym<f64, Variables> {
    let name = sym.clone();
    let f: Rc<Fn(&mut Vec<f64>, &mut Variables)> =
        Rc::new(move |stack: &mut Vec<f64>, map: &mut Variables| {
            stack.push(*map.get(&name).unwrap());
    });
    Sym { name: sym, arity: Arity::new(0, 1), fun: f }
}

pub fn node<A: 'static + Clone, B: 'static + Clone>(sym: Sym<A, B>) -> Sym<Node<A, B>, B> {
    let name = sym.name.clone();
    let num_in = sym.arity.num_in;
    let f: Rc<Fn(&mut Vec<Node<A, B>>, &mut B)> =
        Rc::new(move |stack: &mut Vec<Node<A, B>>, _state: &mut B| {
            let mut children = Vec::new();
            if num_in == 0 {
                stack.push(Node::Leaf(sym.clone()))
            } else {
                for _ in 0..num_in {
                    children.push(stack.pop().unwrap());
                }
                stack.push(Node::Node(sym.clone(), children));
            }
        });
    Sym::new(name, Arity::new(num_in, 1), f)
}


#[derive(Debug, Clone)]
pub struct InstrState {
    pub reg_a: f64,
    pub reg_b: f64,
    pub mem: Vec<f64>,
    pub output: Vec<f64>,
}

impl Default for InstrState {
    fn default() -> InstrState {
        InstrState {
            reg_a: 0.0,
            reg_b: 0.0,
            mem: vec!(0.0, 0.0, 0.0, 0.0, 0.0),
            output: Vec::new(),
        }
    }
}

pub fn store_a() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let arg = stack.pop().unwrap();
            state.reg_a = arg;
    });
    Sym { name: "sa".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn load_a() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            stack.push(state.reg_a);
    });
    Sym { name: "la".to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn store_b() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let arg = stack.pop().unwrap();
            state.reg_b = arg;
    });
    Sym { name: "sb".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn load_b() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            stack.push(state.reg_b);
    });
    Sym { name: "lb".to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn printout() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            state.output.push(stack.pop().unwrap());
    });
    Sym { name: "p".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn store_mem() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let addr = stack.pop().unwrap();
            let arg = stack.pop().unwrap();
            if addr >= 0.0 && (addr as usize) < state.mem.len() {
                state.mem[addr as usize] = arg;
            }
    });
    Sym { name: "sm".to_string(), arity: Arity::new(2, 0), fun: f }
}

pub fn load_mem() -> Sym<f64, InstrState> {
    let f: Rc<Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let addr = stack.pop().unwrap();
            let arg = stack.pop().unwrap();
            if addr >= 0.0 && (addr as usize) < state.mem.len() {
                stack.push(state.mem[addr as usize]);
            }
    });
    Sym { name: "lm".to_string(), arity: Arity::new(2, 1), fun: f }
}

