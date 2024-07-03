use std::net::{IpAddr, ToSocketAddrs};
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
