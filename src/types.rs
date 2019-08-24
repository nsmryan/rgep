

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ind<T>(pub Vec<T>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pop(pub Vec<Ind<u8>>);

