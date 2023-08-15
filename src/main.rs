use icmp;
use std::net::{IpAddr, Ipv4Addr};
use std::{env::args, process::exit};

fn main() {
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("Welcome to nec, network scanner tool");
        exit(1);
    }

    let network: Vec<&str> = args[1].split("/").collect();
    let network_direction = network[0];
    let subnet_mask = network[1];

    println!("\nNetwork direction: {}", network_direction);
    println!("Subnet mask: {}\n", subnet_mask);

    let network_segments: Vec<String> = network_direction
        .split(".")
        .map(|segment| segment.to_string())
        .collect();
    let subnet_mask = subnet_mask.parse::<i32>().unwrap() / 8;

    let network_prefix = &network_segments[..subnet_mask as usize];
    let subnet = &network_segments[subnet_mask as usize..].to_vec();

    search_ips(network_prefix, subnet);
}

fn search_ips(network_prefix: &[String], subnet: &Vec<String>) {
    let mut ips: Vec<String> = Vec::new();

    for i in 1..255 {
        let mut current_subnet = subnet.clone();
        current_subnet[0] = i.to_string();
        if subnet.len() > 1 {
            for i in 1..255 {
                current_subnet[1] = i.to_string();
                if subnet.len() > 2 {
                    for i in 1..255 {
                        current_subnet[2] = i.to_string();
                        // check if the ip is valid
                        let ip =
                            format!("{}.{}", network_prefix.join("."), current_subnet.join("."));
                        if alive_ip(&ip) {
                            ips.push(ip);
                        }
                    }
                }
                // check if the ip is valid
                let ip = format!("{}.{}", network_prefix.join("."), current_subnet.join("."));
                if alive_ip(&ip) {
                    ips.push(ip);
                }
            }
        }
        // check if the ip is valid
        let ip = format!("{}.{}", network_prefix.join("."), current_subnet.join("."));
        if alive_ip(&ip) {
            ips.push(ip);
        }
    }
    println!("Alive ips: {:?}", ips);
}

fn alive_ip(ip: &str) -> bool {
    let ip = ip.parse::<IpAddr>().unwrap();
    let ping = icmp::IcmpSocket::connect(ip);
    let mut ping = ping.unwrap();

    let payload: &[u8] = &[1, 2];

    let result = ping.send(payload);

    match result.ok() {
        Some(_) => true,
        None => false,
    }
}
