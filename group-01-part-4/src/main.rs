mod target;
mod utils;

use std::net::IpAddr;
use target::Target;

/// Main function to test the Target struct
/// This function creates a new target with an IP address, adds a few ports and checks if they are open.
fn main() {
    // Creating a new target with an IP address
    let ip_address: IpAddr = "10.33.1.24".parse().expect("Invalid IP address");
    let mut target = Target::new(ip_address);

    // Displaying the IP address
    println!("IP Address: {}", target.ip_addr());

    // Displaying the initial availability state of the target
    println!("Target online?: {}\n", target.is_up());

    // Adding a few ports and checking if they are open
    target.add_open_port(80);
    target.add_open_port(443);
    target.add_open_port(22);
    println!("Open ports: {:?}", target.open_ports());
}
