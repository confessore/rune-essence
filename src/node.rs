use super::*;

pub struct Node {
    pub peers: Vec<Peer>
}

impl Node {
    pub fn new() -> Self {
        Node {
            peers: vec![]
        }
    }
}
