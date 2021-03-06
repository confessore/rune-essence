use super::*;

pub struct Account {
    pub address: Vec<u8>
    pub pub_key: String,
    priv_key: String,
    pub value: u128,
}

impl Account {
    pub fn new() -> Self {
        Account {

        }
    }
}

impl Hashable for Account {
    fn bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.extend(&self.pub_key.as_bytes());
        bytes.extend(&self.priv_key.as_bytes());
        bytes.extend(&u128_to_bytes(&self.value));
        bytes
    }
}
