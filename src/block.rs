use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u32,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub nonce: u64,
    pub transactions: Vec<Transaction>,
    pub difficulty: u128
}

impl Block {
    pub fn new(
        index: u32,
        timestamp: u128,
        prev_hash: Hash,
        transactions: Vec<Transaction>,
        difficulty: u128) -> Self {
            Block {
                index,
                timestamp,
                hash: vec![0; 32],
                prev_hash,
                nonce: 0,
                transactions,
                difficulty
            }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u64::max_value()) {
            self.nonce = nonce_attempt;
            /*let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }*/
        }
    }
}

impl Hashable for Block {
    /*fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u32_to_bytes(&self.index));
    }*/
}