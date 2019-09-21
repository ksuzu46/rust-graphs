/*******************************************************************************
Title           : edges.rs
Author          : Keisuke Suzuki
Created on      : 9/21/19
Modification    :
*******************************************************************************/
use std::cmp::Ordering;

#[derive(Debug, Copy, Clone, Hash, Eq)]
pub struct E {
    pub(crate) src: usize,
    pub(crate) dest: usize,
    pub(crate) weight: i32
}


//edge impl
impl E {
    pub fn new() -> E { E { src: 0, dest: 0, weight: 0 } }
    pub fn with(src: usize, dest: usize, weight: i32) -> E {
        E { src, dest, weight }
    }
}

impl Ord for E {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl PartialOrd for E {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for E {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}