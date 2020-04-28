use std::rc::Rc;


pub type Stage<State, R> = Rc<dyn Fn(&State, &mut R)>;

pub type StageTransformer<State, R> = Rc<dyn Fn(Stage<State, R>) -> Stage<State, R>>;

pub fn compose_stages<S, R>(stage1: Stage<S, R>, stage2: Stage<S, R>) -> Stage<S, R> 
    where S: 'static ,
          R: 'static {
    let f: Rc<dyn Fn(&S, &mut R)> = Rc::new(move |state, rng| {
        stage1(state, rng);
        stage2(state, rng);
    });

    return f;
}

pub type Getter<S, D> = Rc<dyn Fn(&S) -> D>;

