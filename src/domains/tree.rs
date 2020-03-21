use std::rc::Rc;

use domains::symbols::*;


#[derive(Clone)]
pub enum Node<A, B> {
    Node(Sym<A, B>, Vec<Node<A, B>>),
    Leaf(Sym<A, B>)
}

impl<A: Clone, B: Clone> Node<A, B> {
    pub fn linearize(&self) -> Vec<Sym<A, B>> {
        let mut syms = Vec::new();

        self.linearize_helper(&mut syms);

        syms
    }

    pub fn linearize_helper(&self, syms: &mut Vec<Sym<A, B>>) {
        match self {
            Node::Leaf(sym) => {
                syms.push(sym.clone());
            },

            Node::Node(sym, children) => {
                for node in children.iter().rev() {
                    node.linearize_helper(syms);
                }
                syms.push(sym.clone());
            },
        }
    }

    pub fn eval(&self, state: &mut B) -> A {
        let mut stack = Vec::new();

        match self {
            Node::Leaf(sym) => {
                assert!(sym.arity.num_in == 0);
                assert!(sym.arity.num_out == 1);
                (sym.fun)(&mut stack, state);
                stack.pop().unwrap()
            },

            Node::Node(sym, children) => {
                for child in children {
                    stack.push(child.eval(state));
                }

                (sym.fun)(&mut stack, state);

                stack.pop().unwrap()
            },
        }
    }

    pub fn is_leaf(&self) -> bool {
        match self {
            Node::Leaf(_) => true,
            _ => false,
        }
    }

    pub fn sym(&self) -> Sym<A, B> {
        match self {
            Node::Leaf(sym) => sym.clone(),
            Node::Node(sym, _) => sym.clone(),
        }
    }
}

/// Turn a symbol of any type into a symbol that builds a tree.
/// This allows analysis of the resulting expression.
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

