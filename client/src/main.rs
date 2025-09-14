use std::net::{IpAddr, SocketAddr, UdpSocket};

use reqwest::{Url, blocking::Response};
use stunclient::just_give_me_the_udp_socket_and_its_external_address as socket_and_addr;
use rand::prelude::*;

use crate::peer::{Peer, PeerMessageType};

mod peer;

struct Server {
    socket: UdpSocket,
    address: SocketAddr,
    tracker: Url,
    peers: Vec<Peer>,
}

impl Server {
    pub fn new(tracker: Url) -> Self {
        let (socket, address) = socket_and_addr();

        let peers = Vec::new();
        Server {
            socket,
            address,
            tracker,
            peers,
        }
    }

    pub fn start(mut self) {
        self.peers.append(&mut self.get_peers().unwrap());

        self.announce_to_peers();

        let buf: &mut [u8] = &mut [0; 500];
        loop {
            let (len, peer_addr) = self.socket.recv_from(buf).unwrap();
            self.recv_message(buf, peer_addr, len);
        }
    }

    fn get_peers(&self) -> Option<Vec<Peer>> {
        let http_client = reqwest::blocking::Client::new();
        let body = format!(
            "{{\"ip\":\"{}\", \"port\":{} }}",
            self.address.ip(),
            self.address.port()
        );

        let mut announce_path = self.tracker.clone();
        announce_path.set_path("announce");

        let res = http_client
            .post(announce_path)
            .body(body.clone())
            .send();

        if res.is_err() {
            return None;
        }

        return Some(Server::get_peers_from_http_response(res.unwrap()));
    }

    fn announce_to_peers(&self) {
        for peer in &self.peers {
            let buf = [67];
             self.socket.send_to(&buf, peer.addr);
        }        
    }

    pub fn send_choice(&self, choice: bool) {
        let mut rng = rand::rng();
        let player_count = self.peers.len();
        
        let mut numbers: Vec<u32> = vec![0; player_count];
        numbers = numbers.iter().map(|num| {
            rng.random::<u32>() + (choice as u32)
        }).collect();

        for (index, peer) in self.peers.iter().enumerate() {
            let buf:Vec<u8> = vec![5];
            self.socket.send_to(buf.as_slice(), peer.addr);
        }
        
    }

    fn recv_message(&mut self, buf: &[u8], peer_addr: SocketAddr, len: usize) {
        let peer = self.peers.iter_mut().find(|peer| peer.addr.ip() == peer_addr.ip());

        if peer.is_none() {
            if buf[0] == PeerMessageType::Announcement {
                self.peers.push(Peer::new(peer_addr));
                return;
            }
        };

        let peer = peer.unwrap();

        let value = u32::from_be_bytes(buf[0..4].try_into().unwrap());
        peer.latest_value = Some(value)
    }

    // Fix the unwrap situation eventually
    fn get_peers_from_http_response(res: Response) -> Vec<Peer> {
        let json: serde_json::Value = res.json().unwrap();
        println!("{}", json);

        return json
            .as_object()
            .unwrap()
            .get_key_value("peers")
            .unwrap()
            .1
            .as_array()
            .unwrap()
            .iter()
            .map(|val| {
                // Spamming unwraps is a valid strategy right?
                let ip = val
                    .as_object()
                    .unwrap()
                    .get("ip")
                    .unwrap()
                    .as_str()
                    .unwrap()
                    .parse::<IpAddr>()
                    .unwrap();
                let port = val
                    .as_object()
                    .unwrap()
                    .get("port")
                    .unwrap()
                    .as_u64()
                    .unwrap();
                let port = port as u16;
                Peer::new(SocketAddr::new(ip, port))
            })
            .collect::<Vec<_>>();
    }
}

fn main() {
    let tracker = Url::parse("http://127.0.0.1:3000").unwrap();
    let server = Server::new(tracker);
    server.start();
}
