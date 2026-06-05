// Import clap to handle command line arguments
use clap::{App, Arg};
// Import the scanner module and the scan_range_port function
mod scanner;
use scanner::scan_range_port;

/// Main function
/// This function handles command-line arguments and calls scan_range_port to scan the specified ports on the given IP address.
/// The expected parameters are the IP address and the ports to scan.
fn main() {
    // Define command-line arguments and help messages
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("ESGI")
        .about("Scans a specific range of ports on a given IP address")
        .arg(Arg::new("ip")
            .help("IP address to scan")
            .required(true)
            .index(1))
        .arg(Arg::new("ports")
            .help("Ports to scan, can be a comma-separated list or a range with a dash")
            .required(true)
            .index(2))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Displays all ports, both open and closed")
            .takes_value(false))
        .get_matches();

    // Retrieve argument values
    let ip = matches.value_of("ip").unwrap();
    let ports_str = matches.value_of("ports").unwrap();
    let show_all = matches.is_present("debug");

    // Parse ports
    let ports = match scanner::parse_ports(ports_str) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Error parsing ports: {}", e);
            return;
        }
    };

    // Call the scan_range_port function and display results
    let results = scan_range_port(ip, &ports);

    // Display results
    for (port, is_open) in results {
        if is_open {
            println!("Port {} on {} is open.", port, ip);
        } else if show_all {
            println!("Port {} on {} is closed.", port, ip);
        }
    }
}
