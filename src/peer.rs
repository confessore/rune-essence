use super::*;

pub struct Peer {
    pub node: Node,
    pub peers: Vec<Peer>
}

impl Peer {
    pub new() -> self {
        Peer {
            node: Node,
            peers: vec![]
        }
    }
}
