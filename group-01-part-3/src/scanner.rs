// Import necessary libraries
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::num::ParseIntError;

/// Function to scan a port on a given IP address
/// This function attempts to connect to the specified port on the given IP address.
pub fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Function to scan a range of ports on a given IP address
/// This function calls scan_port for each port in the specified list.
pub fn scan_range_port(ip: &str, ports: &[u16]) -> Vec<(u16, bool)> {
    // Create a vector to store the results
    let mut results = Vec::new();
    // For each port in the list, call scan_port and store the result
    for &port in ports {
        let is_open = scan_port(ip, port);
        results.push((port, is_open));
    }
    results
}

/// Function to parse a string containing a list of ports
/// This function takes a string containing a list of ports separated by commas or dashes.
pub fn parse_ports(ports_str: &str) -> Result<Vec<u16>, ParseIntError> {
    // Create a vector to store the ports
    let mut ports = Vec::new();
    // For each part of the string split by commas, parse individual ports or ranges
    for part in ports_str.split(',') {
        if part.contains('-') {
            let mut range_iter = part.split('-');
            let start = range_iter.next().unwrap().parse::<u16>()?;
            let end = range_iter.next().unwrap().parse::<u16>()?;
            for port in start..=end {
                ports.push(port);
            }
        } else {
            ports.push(part.parse::<u16>()?);
        }
    }
    Ok(ports)
}
