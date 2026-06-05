use std::net::{IpAddr, ToSocketAddrs, SocketAddr, TcpStream};
use std::time::Duration;
use ping::ping;

/// Function to check if a host is online
pub fn host_is_up(address: &str) -> bool {
    // Convert the address to IpAddr
    let ip_addr: IpAddr = match address.parse() {
        Ok(ip) => ip,
        Err(_) => {
            match (address, 0).to_socket_addrs() {
                Ok(mut addrs) => match addrs.next() {
                    Some(addr) => addr.ip(),
                    None => {
                        println!("Connection to address {} is impossible", address);
                        return false;
                    },
                },
                Err(_) => {
                    println!("Error {}", address);
                    return false;
                },
            }
        }
    };

    // Ping verification
    match ping(ip_addr, None, None, None, None, None) {
        Ok(_) => true,
        Err(_err) => {
            println!("Ping failed for {}", ip_addr);
            false
        },
    }
}

/// Function to scan a port
pub fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}
