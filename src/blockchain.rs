use super::*;

pub struct Blockchain {
    pub height: u128,
    pub blocks: Vec<Block>
}

impl Blockchain {
    pub fn new() -> Self {
        Blockchain {
            height: 0,
            blocks: vec![]
        }
    }
}
