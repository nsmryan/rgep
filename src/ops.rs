use std::rc::Rc;

use crate::rgep::*;
use crate::rgep::program::*;
use domains::stack::push_context;


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

pub fn push_context_sym<A: Copy + 'static>() -> Sym<A, A> {
    Sym::new("x".to_string(), Arity::new(0, 2), Rc::new(push_context))
}

pub fn symbol_sym<A: Copy>(sym: String) -> Sym<A, Variables<A>> {
    let name = sym.clone();
    let f: Rc<Fn(&mut Vec<A>, &mut Variables<A>)> =
        Rc::new(move |stack: &mut Vec<A>, map: &mut Variables<A>| {
            stack.push(*map.get(&name).unwrap());
    });
    Sym { name: sym, arity: Arity::new(0, 1), fun: f }
}

