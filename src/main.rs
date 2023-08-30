use dotenv::dotenv;
use std::io;
use std::io::Write;
use std::process::Command;
use std::{env::args, process::exit};

fn main() {
    dotenv().ok();
    let args: Vec<String> = args().collect();

    if args.len() < 2 {
        println!("             \n");
        println!(" _______    ______    _______ ");
        println!("|       \\  /      \\  /       \\");
        println!("| ███████\\|  ██████\\|  ███████");
        println!("| ██  | ██| ██    ██| ██");
        println!("| ██  | ██| ████████| ██_____ ");
        println!("| ██  | ██ \\██     \\ \\██     \\");
        println!(" \\██   \\██  \\███████  \\███████\n");
        println!(
            " network scanner tool v{}",
            std::env::var("NEC_VERSION").expect("NEC_VERSION Needs to ve setted as envar.")
        );
        println!("             \n");
        exit(0);
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
            println!("\r\nFound {} ips\n", ips.len());
            io::stdout().flush().unwrap();
        }
    }
}

fn alive_ip(ip: &str) -> bool {
    print!("\r{}: Tracing", ip);
    io::stdout().flush().unwrap();

    let output = Command::new("ping")
        .arg("-c")
        .arg("1") // Send 1 packet
        .arg("-W")
        .arg("2") // Timeout in seconds
        .arg(&ip)
        .output()
        .expect("Failed to execute ping command");

    if output.status.success() {
        print!("\r{}: UP            \n", ip);
        io::stdout().flush().unwrap()
    }

    output.status.success()
}
