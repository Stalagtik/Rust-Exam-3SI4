// Import clap to handle command line arguments
use clap::{App, Arg};
// Import the scanner module and the scan_port function
mod scanner;
use scanner::scan_port;

/// Main function
/// This function handles command-line arguments and calls scan_port to scan the specified port on the given IP address.
/// The expected parameters are the IP address and the port to scan.
fn main() {
    // Define command-line arguments and help messages
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("ESGI")
        .about("Scans a specific port on a given IP address")
        .arg(Arg::new("ip")
            .help("IP address to scan")
            .required(true)
            .index(1))
        .arg(Arg::new("port")
            .help("Port to scan")
            .required(true)
            .index(2))
        .get_matches();

    // Retrieve argument values
    let ip = matches.value_of("ip").unwrap();
    let port: u16 = matches.value_of_t("port").unwrap_or_else(|e| e.exit());

    // Call the scan_port function and print the result
    let is_open = scan_port(ip, port);

    if is_open {
        // Print a message if the port is open
        println!("Port {} on {} is open.", port, ip);
    } else {
        // Print a message if the port is closed
        println!("Port {} on {} is closed.", port, ip);
    }
}