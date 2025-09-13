use std::net::IpAddr;

use stunclient::{just_give_me_the_udp_socket_and_its_external_address as socket_and_addr};
use zod_rs::prelude::*;

#[derive(Debug, PartialEq)]
struct Peer{
    ip: IpAddr,
    port: u16
}

struct PeerMessage {
    peer: Peer,
    version: u32,
    message: u32
}

impl Peer {
    fn new(ip: IpAddr, port: u16) -> Peer {
        Self { ip, port }
    }
}

fn main() {
    let (udp, addr) = socket_and_addr();

    println!("my_external_addr {}", addr);

    let http_client = reqwest::blocking::Client::new();

    let body = format!("{{\"ip\":\"{}\", \"port\":{} }}", addr.ip(), addr.port());
    let res = http_client.post("http://localhost:3000/announce")
    .body(body.clone()).send();
    println!("{}", body);

    let schema = object().field("peers", array(object().field("ip", string()).field("port", number().int().positive().max(65535.0))))

    let json: serde_json::Value = res.unwrap().json().unwrap();
    println!("{}", json);
    let peers: Vec<Peer> = 
        json.as_object()
        .unwrap()
        .get_key_value("peers")
        .unwrap()
        .1
        .as_array()
        .unwrap()
        .iter()
        .map(|val| {
            // Spamming unwraps is a valid strategy right?
            let ip = val.as_object().unwrap().get("ip").unwrap().as_str().unwrap().parse::<IpAddr>().unwrap();
            let port = val.as_object().unwrap().get("port").unwrap().as_u64().unwrap();
            let port = port as u16;
            Peer::new(ip, port)
        }).collect();

    println!("{:#?}", peers);

    let buf: &mut [u8] = &mut[0; 500];
    while true {
        let (len, peer_addr) = udp.recv_from(buf).unwrap();
        let peer = peers.iter().find(|peer| {
            peer.ip == peer_addr.ip()
        });
        
        if peer.is_none() {
            continue
        }

        

    }
    

}
