mod functions;

use clap::{App, Arg};
use crate::functions::{host_is_up, check_ip_range};

fn main() {
    // Command-line interface configuration using clap
    let matches = App::new("Host Checker")
        .version("1.0")
        .author("TheoOrigin/Jiullian")
        .about("Checks if a host is online or not")
        .arg(Arg::new("address")
            .help("IP address or FQDN of the host to check. To check an address range, use CIDR (e.g. X.X.X.X/24)")
            .required(true)
            .index(1))
        .get_matches();

    // Retrieve user input
    let address = matches.value_of("address").unwrap();

    // Check if address is a CIDR range
    if address.contains('/') {
        check_ip_range(address);
    } else {
        // Check individual address
        let is_up = host_is_up(address);

        if is_up {
            println!("Address: {} is online", address);
        } else {
            println!("Address: {} is offline", address);
        }
    }
}
