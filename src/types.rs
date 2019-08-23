
use std::collections::HashMap;
use std::cmp::max;
use std::rc::Rc;
use std::ops::Add;
use std::iter;
use std::iter::*;

#[cfg(test)]
use float_cmp::*;

use ops::*;


#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Ind<T>(pub Vec<T>);

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Pop(pub Vec<Ind<u8>>);

