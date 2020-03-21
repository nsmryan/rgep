use std::rc::Rc;

use domains::symbols::*;


pub fn push_context<A: Copy>(stack: &mut Vec<A>, b: &mut A) {
    stack.push(*b);
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

