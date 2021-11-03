use super::*;

pub struct Transaction {
    pub id: Hash,
    pub from: Hash,
    pub to: Hash,
    pub value: u128,
    pub fee: u128
}

impl Transaction {
    pub fn new() -> Self {
        Transaction {

        }
    }
}

impl Hashable for Transaction {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.id);
        bytes.extend(&self.from);
        bytes.extend(&self.to);
        bytes.extend(&u128_to_bytes(&self.value));
        bytes.extend(&u128_to_bytes(&self.fee));
        bytes
    }
}
