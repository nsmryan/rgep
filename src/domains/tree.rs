use std::rc::Rc;

use rgep::*;
use rgep::program::*;


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


