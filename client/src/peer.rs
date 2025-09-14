use std::net::SocketAddr;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Peer {
    pub addr: SocketAddr,
    pub latest_value: Option<u32>,
}

impl Peer {
    pub fn new(addr: SocketAddr) -> Peer {
        Self {
            addr,
            latest_value: None,
        }
    }
}

pub enum PeerMessageType {
    Announcement,
    ValueUpdate,
}

pub struct PeerMessage {
    peer: Peer,
    version: u32,
    message: u32,
}

impl PeerMessage {
    pub fn from_bytes(bytes: [u8; 16]) {}
}
