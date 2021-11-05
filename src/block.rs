use super::*;
use std::fmt::{self, Debug, Formatter};

pub struct Block {
    pub index: u128,
    pub timestamp: u128,
    pub hash: Hash,
    pub prev_hash: Hash,
    pub difficulty: u128,
    pub nonce: u128,
    pub transactions: Vec<Transaction>
}

impl Block {
    pub fn new(
        index: u128,
        timestamp: u128,
        prev_hash: Hash,
        difficulty: u128,
        transactions: Vec<Transaction>) -> Self {
            Block {
                index,
                timestamp,
                hash: vec![0; 32],
                prev_hash,
                difficulty,
                nonce: 0,
                transactions
            }
    }

    pub fn mine(&mut self) {
        for nonce_attempt in 0..(u128::max_value()) {
            self.nonce = nonce_attempt;
            let hash = self.hash();
            if check_difficulty(&hash, self.difficulty) {
                self.hash = hash;
                return;
            }
        }
    }
}

impl Hashable for Block {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&u128_to_bytes(&self.index));
        bytes.extend(&u128_to_bytes(&self.timestamp));
        bytes.extend(&self.prev_hash);
        bytes.extend(&u128_to_bytes(&self.difficulty));
        bytes.extend(&u128_to_bytes(&self.nonce));
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>());
        bytes
    }
}

impl Debug for Block {
    fn fmt(&self, formatter: &mut Formatter) -> fmt::Result {
        write!(formatter, "block[{}]: {} at: {} with: {} nonce: {}",
            &self.index,
        &hex::encode(&self.hash),
        &self.timestamp,
        &self.transactions.len(),
        &self.nonce)
    }
}

pub fn check_difficulty(hash: &Hash, difficulty: u128) -> bool {
    difficulty > difficulty_bytes_to_u128(&hash)
}
