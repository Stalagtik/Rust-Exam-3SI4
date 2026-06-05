use std::net::{IpAddr, ToSocketAddrs};
use ipnetwork::IpNetwork;
use ping::ping;

pub fn host_is_up(address: &str) -> bool {
    // Convert address to IpAddr
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

pub fn check_ip_range(ip_range: &str) {
    // Parse CIDR address to IpNetwork
    let ip_network: IpNetwork = match ip_range.parse() {
        Ok(network) => network,
        Err(_) => {
            println!("Invalid CIDR address: {}", ip_range);
            return;
        }
    };

    // Check address type
    match ip_network {
        IpNetwork::V4(v4_network) => {
            let ips = v4_network.iter();
            for ip in ips {
                let octets = ip.octets();
                // Ignore .0 and .255 IPs
                if octets[3] == 0 || octets[3] == 255 {
                    continue;
                }
                
                let ip_addr = IpAddr::V4(ip);
                if host_is_up(&ip_addr.to_string()) {
                    println!("Address {} is online", ip_addr);
                } else {
                    println!("Address {} is offline", ip_addr);
                }
            }
        },
        IpNetwork::V6(_) => {
            println!("IPv6 is not supported for address range checking.");
        }
    }
}
