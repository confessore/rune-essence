use super::*;

pub struct Transaction {
    pub id: Vec<u8>,
    pub from: Vec<u8>,
    pub to: Vec<u8>,
    pub value: u128,
    pub fee: u128
}

impl Transaction {
    pub fn new(
        from: Vec<u8>,
        to: Vec<u8>,
        value: u128) -> Self {
        Transaction {
            id: vec![0; 32],
            from,
            to,
            value,
            fee: 0
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
