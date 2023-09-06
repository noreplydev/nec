use socket2::{Domain, SockAddr, Socket};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

fn main() {
    // CREATE A RAW SOCKET
    let raw_socket = Socket::new(
        Domain::IPV4,
        socket2::Type::RAW,
        Some(socket2::Protocol::ICMPV4),
    )
    .unwrap();

    // SOURCE IP ADDRESS
    let localhost = Ipv4Addr::LOCALHOST;
    let socket_ip = SocketAddr::new(IpAddr::V4(localhost), 0);
    let socket2_ip = SockAddr::from(socket_ip);

    // BIND RAW SOCKET AND SOURCE IP ADDRESS
    raw_socket
        .bind(&socket2_ip)
        .expect(&format!("Failed binding to Ipv4 address {:?}", &socket2_ip));

    println!("Socket: {:?}", raw_socket);
}
