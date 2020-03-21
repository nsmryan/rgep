use std::rc::Rc;

use domains::symbols::*;


pub fn make_const<A: 'static + ToString + Copy, B: 'static>(constant: A) -> Sym<A, B> {
    let f: Rc<dyn Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        stack.push(constant);
    });
    Sym::new(constant.to_string(), Arity::new(0, 1), f)
}

pub fn make_binary<A, B>(name: &str, f: Rc<dyn Fn(A, A) -> A>) -> Sym<A, B>
    where A: 'static + ToString + Copy, B: 'static {
    let f: Rc<dyn Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        let arg1 = stack.pop().unwrap();
        let arg2 = stack.pop().unwrap();
        stack.push(f(arg1, arg2));
    });
    Sym::new(name.to_string(), Arity::new(2, 1), f)
}

pub fn make_unary<A, B>(name: &str, f: Rc<dyn Fn(A) -> A>) -> Sym<A, B>
    where A: 'static + ToString + Copy, B: 'static {
    let f: Rc<dyn Fn(&mut Vec<A>, &mut B)> = Rc::new(move |stack, _context| {
        let arg = stack.pop().unwrap();
        stack.push(f(arg));
    });
    Sym::new(name.to_string(), Arity::new(1, 1), f)
}

