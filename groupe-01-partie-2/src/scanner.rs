//Importer les bibliothèques nécessaires
use std::net::{TcpStream, SocketAddr};
use std::time::Duration;

/// Fonction pour scanner un port sur une adresse IP donnée
/// Cette fonction tente de se connecter au port spécifié sur l'adresse IP donnée.
/// Si la connexion réussit, le port est considéré comme ouvert, sinon il est considéré comme fermé.
pub fn scan_port(ip: &str, port: u16) -> bool {
    //Créer une chaîne de caractères contenant l'adresse IP et le port
    let address = format!("{}:{}", ip, port);
    //Convertir la chaîne en SocketAddr
    let socket_addr: SocketAddr = address.parse().unwrap();
    //Définir un timeout de 3 secondes pour la connexion
    let timeout = Duration::from_secs(3);
    //Tenter de se connecter au port en prenant en compte le timeout spécifié
    match TcpStream::connect_timeout(&socket_addr, timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}
