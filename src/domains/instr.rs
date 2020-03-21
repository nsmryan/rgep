use std::rc::Rc;

use rgep::program::*;


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

