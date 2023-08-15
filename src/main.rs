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
    print!("Subnet mask: {}\n", subnet_mask);
}
