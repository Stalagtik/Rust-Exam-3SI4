mod target;
mod utils;

use std::net::IpAddr;
use target::Target;

fn main() {
    // Création d'une nouvelle cible avec une adresse IP
    let ip_address: IpAddr = "10.33.1.24".parse().expect("Adresse IP invalide");
    let mut target = Target::new(ip_address);

    // Affichage de l'adresse IP
    println!("Adresse IP : {}", target.ip_addr());

    // Affichage de l'état initial de disponibilité de la cible
    println!("Cible en ligne ? : {}\n", target.is_up());

    // Ajout de quelques ports et vérification s'ils sont ouverts
    target.add_open_port(80);
    target.add_open_port(443);
    target.add_open_port(22);
    println!("Ports 'ouverts' : {:?}", target.open_ports());
}
