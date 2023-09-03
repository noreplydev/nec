use pnet;

use pnet::datalink::Channel::Ethernet;
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EthernetPacket, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};

fn main() {
    let nic = pnet::datalink::interfaces()
        .into_iter()
        .find(|iface| iface.name == "eth0" || iface.name == "en0");

    let nic = match nic {
        Some(nic) => nic,
        None => {
            println!("Error: No such interface found");
            return;
        }
    };

    // Create a new channel
    let (mut tx, mut rx) = match datalink::channel(&nic, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    // Create a packet that we will send out
    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                println!(
                    "Packet from {} to {}",
                    packet.get_source(),
                    packet.get_destination()
                );
            }
            Err(e) => {
                // If an error occurs, we can handle it here
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
