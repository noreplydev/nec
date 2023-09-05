use socket2::{Domain, Socket};

fn main() {
    // again, start from 0. I can't achieve an icmp implementation
    let raw_socket = Socket::new(
        Domain::IPV4,
        socket2::Type::RAW,
        Some(socket2::Protocol::ICMPV4),
    )
    .unwrap();

    println!("Hello, world!");
}
