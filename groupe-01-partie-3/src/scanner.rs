//Importer les bibliothèques nécessaires
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::num::ParseIntError;

//Scanne un port sur l'adresse IP donnée et retourne vrai si le port est ouvert, faux sinon
pub fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

//Scanne une plage de ports sur l'adresse IP donnée et retourne une liste (port, ouvert)
pub fn scan_range_port(ip: &str, ports: &[u16]) -> Vec<(u16, bool)> {
    //Créer un vecteur pour stocker les résultats
    let mut results = Vec::new();
    //Pour chaque port dans la liste, appeler scan_port et stocker le résultat
    for &port in ports {
        let is_open = scan_port(ip, port);
        results.push((port, is_open));
    }
    results
}

//Parse les ports à scanner à partir d'une chaîne de caractères
pub fn parse_ports(ports_str: &str) -> Result<Vec<u16>, ParseIntError> {
    //Créer un vecteur pour stocker les ports
    let mut ports = Vec::new();
    //Pour chaque partie de la chaîne, séparée par des virgules ou des tirets, ajouter les ports au vecteur
    for part in ports_str.split(',') {
        if part.contains('-') {
            let mut range_iter = part.split('-');
            let start = range_iter.next().unwrap().parse::<u16>()?;
            let end = range_iter.next().unwrap().parse::<u16>()?;
            for port in start..=end {
                ports.push(port);
            }
        } else {
            ports.push(part.parse::<u16>()?);
        }
    }
    Ok(ports)
}
