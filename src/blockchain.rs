use super::*;
use std::collections::HashSet;

pub struct Blockchain {
    pub height: u128,
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Vec<u8>>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            height: 0,
            blocks: vec![],
            unspent_outputs: HashSet::new()
        }
    }
}
