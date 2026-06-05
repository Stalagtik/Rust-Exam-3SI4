// Import necessary libraries
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

/// Function to scan a port on a given IP address
/// This function attempts to connect to the specified port on the given IP address.
/// If the connection succeeds, the port is considered open; otherwise, it is considered closed.
pub fn scan_port(ip: &str, port: u16) -> bool {
    // Create a string containing the IP address and the port
    let address = format!("{}:{}", ip, port);
    // Parse the string into a SocketAddr
    let socket_addr: SocketAddr = address.parse().unwrap();
    // Set a timeout of 3 seconds for the connection
    let timeout = Duration::from_secs(3);
    // Attempt to connect to the port using the specified timeout
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}
