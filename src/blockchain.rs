use super::*;
use std::collections::HashSet;

pub struct Blockchain {
    pub blocks: Vec<Block>,
    unspent_outputs: HashSet<Hash>
}
