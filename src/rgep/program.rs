use domains::symbols::*;


pub struct Program<A, B>(pub Vec<Sym<A, B>>);

impl<A, B> Program<A, B> {
    pub fn eval(&self, state: &mut B, default: A) -> A {
        let mut stack = Vec::new();
        self.eval_with_stack(state, default, &mut stack)
    }

    pub fn eval_with_stack(&self, state: &mut B, default: A, stack: &mut Vec<A>) -> A {
        self.exec_with_stack(state, stack);
        if stack.len() > 0 {
            stack.pop().unwrap()
        } else {
            default
        }
    }

    pub fn exec(&self, state: &mut B) -> Vec<A> {
        let mut stack = Vec::new();
        self.exec_with_stack(state, &mut stack);
        stack
    }

    pub fn exec_with_stack(&self, state: &mut B, stack: &mut Vec<A>) {
        for sym in self.0.iter() {
            if stack.len() >= sym.arity.num_in {
                (sym.fun)(stack, state);
            }
        }
    }

    pub fn to_string(&self) -> String {
        let mut string = "".to_string();

        for sym in self.0.iter() {
            string.push_str(&sym.name);
            string.push_str(&"");
        }

        string
    }
}

