use std::io;
use std::io::Write;
use std::process::Command;
use std::{env::args, process::exit}; // <--- bring flush() into scope

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

    for i in 1..256 {
        let mut current_subnet = subnet.clone();
        current_subnet[0] = i.to_string();
        if subnet.len() > 1 {
            for i in 1..256 {
                current_subnet[1] = i.to_string();
                if subnet.len() > 2 {
                    for i in 1..256 {
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

        if i == 255 {
            println!("\r\nFound {} ips", ips.len());
            io::stdout().flush().unwrap();
        }
    }

    // here are the ips that are alive
}

fn alive_ip(ip: &str) -> bool {
    print!("\r{}: Tracing", ip);
    io::stdout().flush().unwrap();

    let output = Command::new("ping")
        .arg("-c")
        .arg("1") // Send 1 packet
        .arg("-W")
        .arg("1") // Timeout in seconds
        .arg(&ip)
        .output()
        .expect("Failed to execute ping command");

    if output.status.success() {
        print!("\r{}: UP       \n", ip);
        io::stdout().flush().unwrap()
    }
    output.status.success()
}
