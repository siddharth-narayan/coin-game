use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use stunclient::StunClient;

fn main() {
    let local_addr: SocketAddr = "0.0.0.0:0".parse().unwrap();
    let stun_addr = "stun.l.google.com:19302"
        .to_socket_addrs()
        .unwrap()
        .filter(|x| x.is_ipv4())
        .next()
        .unwrap();
    let udp = UdpSocket::bind(local_addr).unwrap();

    let c = StunClient::new(stun_addr);

    let my_external_addr: SocketAddr = c.query_external_address(&udp).unwrap();

    println!("my_external_addr {}", my_external_addr);

    let my_external_addr: SocketAddr = c.query_external_address(&udp).unwrap();

    println!("my_external_addr {}", my_external_addr);

    let http_client = reqwest::blocking::Client::new();

    let body = format!("{{\"ip\":\"{}\", \"port\":{} }}", my_external_addr.ip(), my_external_addr.port());
    let res = http_client.post("http://localhost:3000/announce")
    .body(body.clone()).send();
    println!("{}", body);
}
