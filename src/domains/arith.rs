use std::rc::Rc;
use std::fmt::Display;

use num::Num;
use num::FromPrimitive;

use ops::*;

use rgep::*;
use rgep::program::*;


#[derive(Clone, PartialEq)]
pub enum Arith<A> {
    Add(Box<Arith<A>>, Box<Arith<A>>),
    Sub(Box<Arith<A>>, Box<Arith<A>>),
    Mult(Box<Arith<A>>, Box<Arith<A>>),
    Div(Box<Arith<A>>, Box<Arith<A>>),
    Const(A),
    Var(String),
}

impl<A> Arith<A> 
    where A: Num + ToString + Display + FromPrimitive + Copy {
    pub fn eval(&self, context: &Variables<A>) -> A {
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
                if denom == A::from_f64(0.0).unwrap() {
                    A::from_f64(0.0).unwrap()
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

    pub fn is_constant(&self, test_contant: A) -> bool {
        match self {
            Arith::Const(constant) => *constant == test_contant,
            _ => false,
        }
    }

    pub fn simplify(&self) -> Arith<A> {
        match self.clone() {
            Arith::Add(exp1, exp2) => {
                let exp1 = exp1.simplify();
                let exp2 = exp2.simplify();

                if exp1.is_const() && exp2.is_const() {
                    match (exp1, exp2) {
                        (Arith::Const(c1), Arith::Const(c2)) => Arith::Const(c1 + c2),
                        _ => panic!("Simplify sum should not have reached this code!"),
                    }
                } else if exp1.is_constant(A::from_f64(0.0).unwrap()) {
                    exp2
                } else if exp2.is_constant(A::from_f64(0.0).unwrap()) {
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
                } else if exp1.is_constant(A::from_f64(0.0).unwrap()) || exp2.is_constant(A::from_f64(0.0).unwrap()) {
                    Arith::Const(A::from_f64(0.0).unwrap())
                } else if exp1.is_constant(A::from_f64(1.0).unwrap()) {
                    exp2
                } else if exp2.is_constant(A::from_f64(1.0).unwrap()) {
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

                if exp2.is_constant(A::from_f64(0.0).unwrap()) {
                    Arith::Const(A::from_f64(0.0).unwrap())
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

pub fn add_expr<A>() -> Sym<Arith<A>, Variables<A>> {
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Add(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "+".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn sub_expr<A>() -> Sym<Arith<A>, Variables<A>> {
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Sub(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "-".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn div_expr<A>() -> Sym<Arith<A>, Variables<A>> {
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Div(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "/".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn mult_expr<A>() -> Sym<Arith<A>, Variables<A>> {
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            let arg1 = stack.pop().unwrap();
            let arg2 = stack.pop().unwrap();
            stack.push(Arith::Mult(Box::new(arg1), Box::new(arg2)));
    });
    Sym { name: "*".to_string(), arity: Arity::new(2, 1), fun: f }
}

pub fn const_expr<A>(constant: A) -> Sym<Arith<A>, Variables<A>>
    where A: Num + Display + 'static + Copy {
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            stack.push(Arith::Const(constant));
    });
    Sym { name: constant.to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn var_expr<A>(name: String) -> Sym<Arith<A>, Variables<A>> {
    let sym_name = name.clone();
    let f: Rc<Fn(&mut Vec<Arith<A>>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<Arith<A>>, _map: &mut Variables<A>| {
            stack.push(Arith::Var(name.clone()));
    });
    Sym { name: sym_name, arity: Arity::new(0, 1), fun: f }
}

pub fn zero_sym<A, B:'static>() -> Sym<A, B>
    where A: Num + Display + 'static + FromPrimitive + Copy,
          B: 'static {
    make_const(FromPrimitive::from_u32(0).unwrap())
}

pub fn one_sym<A, B:'static>() ->  Sym<A, B>
    where A: Num + Display + 'static + FromPrimitive + Copy,
          B: 'static {
    make_const(FromPrimitive::from_u32(1).unwrap())
}

pub fn two_sym<A, B:'static>() ->  Sym<A, B>
    where A: Num + Display + 'static + FromPrimitive + Copy,
          B: 'static {
    make_const(FromPrimitive::from_u32(2).unwrap())
}

pub fn plus_sym<A, B>() -> Sym<A, B> 
    where A: Num + Display + 'static + Copy,
          B: 'static {
    make_binary("+", Rc::new(|a, b| a + b))
}

pub fn sub_sym<A, B>() -> Sym<A, B>
    where A: Num + Display + 'static + Copy,
          B: 'static {
    make_binary("-", Rc::new(|a, b| a - b))
}

pub fn mult_sym<A, B>() -> Sym<A, B>
    where A: Num + Display + 'static + Copy,
          B: 'static {
    make_binary("*", Rc::new(|a, b| a * b))
}

pub fn mod_sym<A, B>() -> Sym<A, B>
    where A: Num + Display + 'static + Copy,
          B: 'static {
    make_binary("%", Rc::new(|a, b| if b != A::zero() { a % b } else { A::zero() } ))
}

pub fn div_sym<A, B>() -> Sym<A, B>
    where A: Num + Display + 'static + Copy,
          B: 'static {
    make_binary("/", Rc::new(|a, b| {
        if b == A::zero() {
            A::zero()
        } else {
            a / b
        }
    }))
}
