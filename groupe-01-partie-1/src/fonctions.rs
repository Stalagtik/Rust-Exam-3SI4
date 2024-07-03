use std::net::{IpAddr, ToSocketAddrs};
use ping::ping;

pub fn host_is_up(address: &str) -> bool {
    //passage de l'adresse en ip
    let ip_addr: IpAddr = match address.parse() {
        //si c'est un ip valide, on l'utilise
        Ok(ip) => ip,
        //si elle est pas valide, on essaie de la passer en nom de domaine
        Err(_) => {
            //utiliser to_socket_addrs pour trouver l'adresse
            match (address, 0).to_socket_addrs() {
                //obtenir la première adresse trouvé
                Ok(mut addrs) => match addrs.next() {
                    //on utilise l'adresse ip trouvé
                    Some(addr) => addr.ip(),
                    //si aucune adresse est trouvée return false
                    None => {
                        println!("connexion a l'adresse : {} impossible", address);
                        return false;
                    },
                },
                Err(_) => {
                    println!("error {}", address);
                    return false;
                },
            }
        }
    };

    match ping(ip_addr, None, None, None, None, None) {
        //si le ping réussi return true
        Ok(_) => { 
            true
        },
        Err(err) => {
            println!("ping echoué : {}, error: {}", ip_addr, err);
            false
        },
    }
}
