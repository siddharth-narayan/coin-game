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

pub enum PeerMessage {
    Announcement,
    ValueUpdate { new_val: u32 },
    SumAnnouncement { sum: u64 },
}

impl PeerMessage {
    pub fn from_bytes(buf: &[u8]) -> Option<Self> {
        match buf[0] {
            67 => Some(PeerMessage::Announcement),
            68 => Some(PeerMessage::ValueUpdate { new_val: u32::from_be_bytes(buf[0..4].try_into().unwrap()) }),
            69 => Some(PeerMessage::SumAnnouncement { sum: u64::from_be_bytes(buf[0..8].try_into().unwrap()) }),
            _ => None,
        }
    }
    
    pub fn announce() {}
}
