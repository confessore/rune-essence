use super::*;

pub struct Account {
    pub pub_key: Address,
    priv_key: Address,
    pub value: u128,
    pub transactions: Vec<Transaction
}

impl Account {
    pub fn new(&self) -> Self {
        Account {

        }
    }
}

impl Hashable for Account {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.pub_key.as_bytes())
        bytes.extend(&self.priv_key.as_bytes())
        bytes.extend(u128_to_bytes(&self.value))
        bytes.extend(
            self.transactions
                .iter()
                .flat_map(|transaction| transaction.bytes())
                .collect::<Vec<u8>>());
        bytes
    }
}
