use std::net::{IpAddr, ToSocketAddrs, SocketAddr, TcpStream};
use std::time::Duration;
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

pub fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}
