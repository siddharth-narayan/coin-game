use std::net::UdpSocket;
use std::net::{SocketAddr, ToSocketAddrs};
use stunclient::{just_give_me_the_udp_socket_and_its_external_address as socket_and_addr};

fn main() {
    let (udp, addr): (UdpSocket, SocketAddr) = socket_and_addr();

    println!("my_external_addr {}", addr);

    let http_client = reqwest::blocking::Client::new();

    let body = format!("{{\"ip\":\"{}\", \"port\":{} }}", addr.ip(), addr.port());
    let res = http_client.post("http://localhost:3000/announce")
    .body(body.clone()).send();
    println!("{}", body);

    udp.

}
