extern crate rgep;
extern crate rand;

use std::rc::Rc;

use rand::prelude::*;
use rand::rngs::SmallRng;

use rgep::ga::*;
use rgep::types::*;
use rgep::evaluation::*;



fn main() {
    let mut params = GaParams::default();
    params.num_gens *= 10;

    let eval: Eval<IndU8, SmallRng> =
        Rc::new(|ind, rng| {
            let mut sum: f64 = 0.0;
            for value in ind.0.iter() {

                sum += *value as f64;
            }
            return sum;
    });

    let mut rng = SmallRng::from_entropy();

    let pop = ga(&params, eval, &mut rng);

    let mut best = 0.0;
    for ind in (*pop.borrow_mut()).0.iter() {
        let mut current = 0.0;
        for ix in ind.0.iter() {
            current += *ix as f64;
        }

        if current > best {
            best = current;
        }
    }

    println!("best fitness = {}", best);
}
