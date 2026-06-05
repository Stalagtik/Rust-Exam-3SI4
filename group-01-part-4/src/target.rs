use std::net::IpAddr;

use crate::utils::host_is_up;
use crate::utils::scan_port;


/// Structure representing a target to scan
pub struct Target {
    ip_addr: IpAddr,
    is_up: bool,
    open_ports: Vec<u16>,
}

/// Implementation of the Target struct
impl Target {
    // Create a new instance of Target
    pub fn new(ip_addr: IpAddr) -> Self {
        Target {
            ip_addr,
            is_up: host_is_up(&ip_addr.to_string()),
            open_ports: Vec::new(),
        }
    }

    // Getter for the IP address of the target
    pub fn ip_addr(&self) -> &IpAddr {
        &self.ip_addr
    }

    // Getter for the availability state of the target
    pub fn is_up(&self) -> bool {
        self.is_up
    }

    // Setter for the availability state of the target
    pub fn set_is_up(&mut self, is_up: bool) {
        self.is_up = is_up;
    }

    // Getter for the open ports of the target
    pub fn open_ports(&self) -> &Vec<u16> {
        &self.open_ports
    }

    // Method to add a scanned open port to the target
    pub fn add_open_port(&mut self, port: u16) {
        if scan_port(&self.ip_addr.to_string(), port) {
            self.open_ports.push(port);
        }
    }
}
