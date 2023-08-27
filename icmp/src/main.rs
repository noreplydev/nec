use pnet::datalink::{self, Channel};
use pnet::packet::ipv4::Ipv4Packet;
use std::process::exit;

fn main() {
    let interfaces = pnet::datalink::interfaces();
    let nic = interfaces
        .into_iter()
        .find(|iface| iface.name == "eth0" || iface.name == "en0");

    let nic = match nic {
        Some(nic) => nic,
        None => {
            println!("Defaults NICs not found. You can specify by using --nic flag.");
            exit(1);
        }
    };

    let (mut tx, mut rx) = match datalink::channel(&nic, Default::default()) {
        Ok(Channel::Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => {
            println!("Unhandled channel type");
            exit(1);
        }
        Err(e) => {
            println!("Failed creating a raw socket. Try with sudo. {}", e);
            exit(1);
        }
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = match Ipv4Packet::new(packet) {
                    Some(packet) => packet,
                    None => {
                        println!("Unknow packet.");
                        continue;
                    }
                };

                if packet.get_version() == 4 {
                    handleIPv4Packet(&packet);
                    continue;
                }
            }
            Err(e) => {
                println!("Failed to read: {}", e);
            }
        }
    }
}

fn handleIPv4Packet(packet: &Ipv4Packet) {
    println!(
        "IPv4 packet: {} -> {}",
        packet.get_source(),
        packet.get_destination()
    );
}
