use std::collections::HashMap;
use std::rc::Rc;
use std::boxed::Box;

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
            Arith::Const(constant) => true,
            Arith::Var(name) => true,
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

pub fn zero<B>(stack: &mut Vec<f64>, _b: &mut B) {
    stack.push(0.0);
}

pub fn one<B>(stack: &mut Vec<f64>, _b: &mut B) {
    stack.push(1.0);
}

pub fn two<B>(stack: &mut Vec<f64>, _b: &mut B) {
    stack.push(2.0);
}

pub fn plus<B>(stack: &mut Vec<f64>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1 + arg2);
}

pub fn mult<B>(stack: &mut Vec<f64>, _b: &mut B) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1 * arg2);
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

pub fn make_const<A: 'static, B: 'static>(name: String, constant: A) -> Sym<A, B> {
    let zero: Fn(&mut Vec<f64>, &mut B) = |stack, context| {
        stack.push(0.0);
    };
    Sym::new(constant.to_string(), Arity::new(0, 1), Rc::new(zero))
}

pub fn zero_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("0".to_string(), Arity::new(0, 1), Rc::new(zero))
}

pub fn one_sym<B:'static>() ->  Sym<f64, B> {
    Sym::new("1".to_string(), Arity::new(0, 1), Rc::new(one))
}

pub fn two_sym<B:'static>() ->  Sym<f64, B> {
    Sym::new("2".to_string(), Arity::new(0, 1), Rc::new(two))
}

pub fn plus_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("+".to_string(), Arity::new(2, 1), Rc::new(plus))
}

pub fn mult_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("*".to_string(), Arity::new(2, 1), Rc::new(mult))
}

pub fn dup_sym<A: 'static + Clone, B:'static>() -> Sym<A, B> {
    Sym::new("dup".to_string(), Arity::new(1, 2), Rc::new(dup))
}

pub fn swap_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("swap".to_string(), Arity::new(2, 2), Rc::new(swap))
}

pub fn drop_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("drop".to_string(), Arity::new(1, 0), Rc::new(drop))
}

pub fn nip_sym<B:'static>() -> Sym<f64, B> {
    Sym::new("drop".to_string(), Arity::new(2, 1), Rc::new(nip))
}

pub fn tuck_sym<B:'static>() -> Sym<f64, B> {
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

