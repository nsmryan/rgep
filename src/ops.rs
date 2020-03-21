use std::rc::Rc;

use domains::symbols::*;
use domains::stack::push_context;
use domains::tree::Node;


pub fn push_context_sym<A: Copy + 'static>() -> Sym<A, A> {
    Sym::new("x".to_string(), Arity::new(0, 2), Rc::new(push_context))
}

pub fn symbol_sym<A: Copy>(sym: String) -> Sym<A, Variables<A>> {
    let name = sym.clone();
    let f: Rc<dyn Fn(&mut Vec<A>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<A>, map: &mut Variables<A>| {
            stack.push(*map.get(&name).unwrap());
    });
    Sym { name: sym, arity: Arity::new(0, 1), fun: f }
}

pub fn node<A: 'static + Clone, B: 'static + Clone>(sym: Sym<A, B>) -> Sym<Node<A, B>, B> {
    let name = sym.name.clone();
    let num_in = sym.arity.num_in;
    let f: Rc<dyn Fn(&mut Vec<Node<A, B>>, &mut B)> =
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
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let arg = stack.pop().unwrap();
            state.reg_a = arg;
    });
    Sym { name: "sa".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn load_a() -> Sym<f64, InstrState> {
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            stack.push(state.reg_a);
    });
    Sym { name: "la".to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn store_b() -> Sym<f64, InstrState> {
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let arg = stack.pop().unwrap();
            state.reg_b = arg;
    });
    Sym { name: "sb".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn load_b() -> Sym<f64, InstrState> {
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            stack.push(state.reg_b);
    });
    Sym { name: "lb".to_string(), arity: Arity::new(0, 1), fun: f }
}

pub fn printout() -> Sym<f64, InstrState> {
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            state.output.push(stack.pop().unwrap());
    });
    Sym { name: "p".to_string(), arity: Arity::new(1, 0), fun: f }
}

pub fn store_mem() -> Sym<f64, InstrState> {
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
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
    let f: Rc<dyn Fn(&mut Vec<f64>, &mut InstrState)> =
        Rc::new(move |stack: &mut Vec<f64>, state: &mut InstrState| {
            let addr = stack.pop().unwrap();
            if addr >= 0.0 && (addr as usize) < state.mem.len() {
                stack.push(state.mem[addr as usize]);
            }
    });
    Sym { name: "lm".to_string(), arity: Arity::new(2, 1), fun: f }
}

