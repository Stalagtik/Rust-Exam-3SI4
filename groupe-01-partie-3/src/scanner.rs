//Importer les bibliothèques nécessaires
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;
use std::num::ParseIntError;

/// Fonction pour scanner un port sur une adresse IP donnée
/// Cette fonction tente de se connecter au port spécifié sur l'adresse IP donnée.
pub fn scan_port(ip: &str, port: u16) -> bool {
    let address = format!("{}:{}", ip, port);
    let socket_addr: SocketAddr = address.parse().unwrap();
    let timeout = Duration::from_secs(3);

    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Fonction pour scanner une plage de ports sur une adresse IP donnée
/// Cette fonction appelle la fonction scan_port pour chaque port dans la liste spécifiée.
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

/// Fonction pour parser une chaîne de caractères contenant une liste de ports
/// Cette fonction prend une chaîne de caractères contenant une liste de ports séparés par des virgules ou des tirets.
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
