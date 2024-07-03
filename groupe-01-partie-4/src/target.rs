use std::net::IpAddr;

pub struct Target {
    ip_addr: IpAddr,
    is_up: bool,
    open_ports: Vec<u16>,
}

impl Target {
    //créer une nouvelle instance de Target
    pub fn new(ip_addr: IpAddr) -> Self {
        Target {
            ip_addr,
            is_up: false,
            open_ports: Vec::new(),
        }
    }

    //getter pour l'adresse IP de la cible
    pub fn ip_addr(&self) -> &IpAddr {
        &self.ip_addr
    }

    //getter pour l'état de disponibilité de la cible
    pub fn is_up(&self) -> bool {
        self.is_up
    }

    //setter pour l'état de disponibilité de la cible
    pub fn set_is_up(&mut self, is_up: bool) {
        self.is_up = is_up;
    }

    //getter pour les ports ouverts de la cible
    pub fn open_ports(&self) -> &Vec<u16> {
        &self.open_ports
    }

    //setter pour ajouter un port ouvert à la cible
    pub fn add_open_port(&mut self, port: u16) {
        self.open_ports.push(port);
    }
}
