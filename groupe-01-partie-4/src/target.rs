use std::net::IpAddr;

use crate::utils::host_is_up;
use crate::utils::scan_port;


/// Structure représentant une cible à scanner
pub struct Target {
    ip_addr: IpAddr,
    is_up: bool,
    open_ports: Vec<u16>,
}

/// Implémentation de la structure Target
impl Target {
    // Créer une nouvelle instance de Target
    pub fn new(ip_addr: IpAddr) -> Self {
        Target {
            ip_addr,
            is_up: host_is_up(&ip_addr.to_string()),
            open_ports: Vec::new(),
        }
    }

    // Getter pour l'adresse IP de la cible
    pub fn ip_addr(&self) -> &IpAddr {
        &self.ip_addr
    }

    // Getter pour l'état de disponibilité de la cible
    pub fn is_up(&self) -> bool {
        self.is_up
    }

    // Setter pour l'état de disponibilité de la cible
    pub fn set_is_up(&mut self, is_up: bool) {
        self.is_up = is_up;
    }

    // Getter pour les ports ouverts de la cible
    pub fn open_ports(&self) -> &Vec<u16> {
        &self.open_ports
    }

    // Setter pour ajouter un port ouvert à la cible
    pub fn add_open_port(&mut self, port: u16) {
        if scan_port(&self.ip_addr.to_string(), port) {
            self.open_ports.push(port);
        }
    }
}
