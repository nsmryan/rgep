use std::rc::Rc;

use ops::*;

use rgep::program::*;


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
    make_unary("-", Rc::new(|a: u32| !a))
}

