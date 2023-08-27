use std::process::exit;

use pnet::datalink::{self, Channel};

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
            println!("Failed creating a raw socket {}", e);
            exit(1);
        }
    };

    loop {
        let mut buf = [0u8; 1600];
        match rx.next() {
            Ok(_) => {
                println!("Received a packet!");
            }
            Err(e) => {
                println!("Failed to read: {}", e);
            }
        }
    }
}
