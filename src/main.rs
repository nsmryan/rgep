extern crate rand;
extern crate statrs;


use std::cmp::max;

use rand::prelude::*;

use statrs::distribution::{Uniform, Continuous};


#[derive(Debug)]
struct Ind(Vec<u8>);

impl Ind {
    fn show(&self, params: &Params<i32>) -> String {
        let mut string = "".to_string();

        for code in self.0.iter() {
            let sym = params.decode(*code);
            string.push_str(&sym.name);
            string.push_str(&"");
        }

        string
    }
    
    fn eval(&self, params: &Params<i32>) -> i32 {
        self.eval_with_stack(params, &Vec::new())
    }

    fn eval_with_stack(&self, params: &Params<i32>, stack: &Vec<i32>) -> i32 {
        let mut local_stack = stack.clone();
        self.execute_with_stack(params, &mut local_stack);
        match local_stack.pop() {
            Some(result) => result,
            None => params.default,
        }
    }

    fn execute(&self, params: &Params<i32>) -> Vec<i32> {
        let mut stack = Vec::new();
        self.execute_with_stack(params, &mut stack);
        stack
    }

    fn execute_with_stack(&self, params: &Params<i32>, stack: &mut Vec<i32>) {
        for code in self.0.iter() {
            let sym = params.decode(*code);
            if stack.len() >= sym.arity.num_in {
                (sym.fun)(stack);
            }
        }
    }
}

#[derive(Debug)]
struct Pop(Vec<Ind>);

impl Pop {
    fn create<A, R>(params: &Params<A>, rng: &mut R) -> Pop 
    where R: Rng, A: Clone {
        let mut pop = Vec::with_capacity(params.pop_size);

        let mut rng = rand::thread_rng();

        let bits_needed = params.bits_per_sym();
        assert!(bits_needed <= 8, "This implementation does not currently support multiple byte symbols");
        let bytes_per_sym = ((bits_needed as f64) / 8.0).ceil() as usize;

        let range = 2_u32.pow(bits_needed as u32);

        for _ in 0..params.pop_size {
            let mut ind_vec = Vec::with_capacity(params.ind_size);
            for _ in 0..params.ind_size {
                ind_vec.push(rng.gen_range(0, range) as u8);
            }
            pop.push(Ind(ind_vec));
        }

        Pop(pop)
    }
}

#[derive(Clone)]
struct Arity {
    num_in:  usize,
    num_out: usize,
}

impl Arity {
    pub fn new(num_in: usize, num_out: usize) -> Self {
        Arity { num_in: num_in, num_out: num_out }
    }
}

// NOTE this should take a state parameter for things like variables, or
// shared execution state in a program.
#[derive(Clone)]
struct Sym<'a, A: Clone + 'a> {
    name: String,
    arity: Arity,
    fun: &'a Fn(&mut Vec<A>),
}

// NOTE this may be split into params and execution configuration
#[derive(Clone)]
struct Params<'a, A: Clone + 'a> {
    prob_mut: f64,
    prob_one_point_crossover: f64,
    prob_two_point_crossover: f64,
    prob_rotation: f64,

    terminals: Vec<Sym<'a, A>>,
    functions: Vec<Sym<'a, A>>,

    default: A,

    pop_size: usize,
    ind_size: usize,
}

impl<'a, A: Clone> Params<'a, A> {
    pub fn num_symbols(&self) -> usize {
        self.terminals.len() + self.functions.len()
    }

    pub fn bits_per_sym(&self) -> usize {
        let syms_to_encode = max(self.terminals.len(), self.functions.len());
        ((syms_to_encode as f64).log2().ceil()) as usize + 1
    }

    pub fn bytes_per_sym(&self) -> usize {
        ((self.bits_per_sym() as f64) / 8.0).ceil() as usize
    }
}

impl<'a> Params<'a, i32> {
    pub fn decode(&self, code: u8) -> Sym<i32> {
        let is_function = (code & 1) == 1;
        let index = (code >> 1) as usize;
        if is_function {
            self.functions[index % self.functions.len()].clone()
        } else {
            self.terminals[index % self.terminals.len()].clone()
        }
    }
}

fn zero(stack: &mut Vec<i32>) {
    stack.push(0);
}

fn one(stack: &mut Vec<i32>) {
    stack.push(1);
}

fn two(stack: &mut Vec<i32>) {
    stack.push(2);
}

fn plus(stack: &mut Vec<i32>) {
    let arg1 = stack.pop().unwrap();
    let arg2 = stack.pop().unwrap();
    stack.push(arg1 + arg2);
}

#[test]
fn test_eval_simple_equation() {
    let zero_sym = Sym { name: "0".to_string(), arity: Arity::new(0, 1), fun: &zero };
    let one_sym  = Sym { name: "1".to_string(), arity: Arity::new(0, 1), fun: &one  };
    let two_sym  = Sym { name: "2".to_string(), arity: Arity::new(0, 1), fun: &two  };
    let terminals =
        vec!(zero_sym, one_sym, two_sym);

    let plus_sym = Sym { name: "+".to_string(), arity: Arity::new(2, 1), fun: &plus };
    let functions = vec!(plus_sym);

    let params: Params<i32> = Params {
        prob_mut: 0.001,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.01,

        terminals: terminals,
        functions: functions,

        default: 0,

        pop_size: 5,
        ind_size: 4,
    };

    let mut ind_vec = Vec::new();
    ind_vec.push(2); // one
    ind_vec.push(4); // two
    ind_vec.push(1); // plus
    let ind = Ind(ind_vec);
    let result = ind.eval(&params);
    assert!(result == 3, format!("result was {}", result))
}

fn main() {
    let terminals =
        vec!(Sym { name: "0".to_string(), arity: Arity::new(0, 1), fun: &zero },
             Sym { name: "1".to_string(), arity: Arity::new(0, 1), fun: &one  },
             Sym { name: "2".to_string(), arity: Arity::new(0, 1), fun: &two  });

    let functions =
        vec!(Sym { name: "+".to_string(), arity: Arity::new(2, 1), fun: &plus });

    let params: Params<i32> = Params {
        prob_mut: 0.001,
        prob_one_point_crossover: 0.6,
        prob_two_point_crossover: 0.6,
        prob_rotation: 0.01,

        terminals: terminals,
        functions: functions,

        default: 0,

        pop_size: 5,
        ind_size: 4,
    };

    println!("bits = {}", params.bits_per_sym());
    println!("bytes = {}", params.bytes_per_sym());
    
    let mut rng = thread_rng();
    let mut pop = Pop::create(&params, &mut rng);
    println!("{:?}", pop);

    for ind in pop.0 {
        println!("{}", ind.show(&params));
    }
}
