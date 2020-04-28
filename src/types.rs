use rand::rngs::SmallRng;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ind<T>(pub Vec<T>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pop<T>(pub Vec<Ind<T>>);

pub type PopU8 = Pop<u8>;
pub type IndU8 = Ind<u8>;

pub type R = SmallRng;

