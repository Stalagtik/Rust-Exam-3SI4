use std::net::{IpAddr, ToSocketAddrs};
use ipnetwork::IpNetwork;
use ping::ping;

pub fn host_is_up(address: &str) -> bool {
    //conversion de l'adresse en IpAddr
    let ip_addr: IpAddr = match address.parse() {
        Ok(ip) => ip,
        Err(_) => {
            match (address, 0).to_socket_addrs() {
                Ok(mut addrs) => match addrs.next() {
                    Some(addr) => addr.ip(),
                    None => {
                        println!("Connexion à l'adresse {} impossible", address);
                        return false;
                    },
                },
                Err(_) => {
                    println!("Erreur {}", address);
                    return false;
                },
            }
        }
    };

    //vérification du ping
    match ping(ip_addr, None, None, None, None, None) {
        Ok(_) => true,
        Err(_err) => {
            println!("Ping échoué pour {}", ip_addr);
            false
        },
    }
}

pub fn check_ip_range(ip_range: &str) {
    //passage de l'adresse CIDR en IpNetwork
    let ip_network: IpNetwork = match ip_range.parse() {
        Ok(network) => network,
        Err(_) => {
            println!("Adresse CIDR invalide : {}", ip_range);
            return;
        }
    };

    //vérification du type d'adresse
    match ip_network {
        IpNetwork::V4(v4_network) => {
            let ips = v4_network.iter();
            for ip in ips {
                let octets = ip.octets();
                //ignore les ip .0 et .255
                if octets[3] == 0 || octets[3] == 255 {
                    continue;
                }
                
                let ip_addr = IpAddr::V4(ip);
                if host_is_up(&ip_addr.to_string()) {
                    println!("Adresse {} est en ligne", ip_addr);
                } else {
                    println!("Adresse {} est hors ligne", ip_addr);
                }
            }
        },
        IpNetwork::V6(_) => {
            println!("IPv6 n'est pas supporté pour la plage d'adresses.");
        }
    }
}
