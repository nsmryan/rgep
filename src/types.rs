

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ind<T>(pub Vec<T>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pop<T>(pub Vec<Ind<T>>);

type PopU8 = Pop<u8>;

