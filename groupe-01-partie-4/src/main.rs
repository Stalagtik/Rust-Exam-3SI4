mod target;

use std::net::IpAddr;
use target::Target;

fn main() {
    //création d'une nouvelle cible avec une adresse IP
    let ip_address: IpAddr = "10.33.24.1".parse().expect("Adresse IP invalide");
    let mut target = Target::new(ip_address);

    //affichage de l'adresse IP
    println!("Adresse IP : {}", target.ip_addr());

    //affichage de l'état initial de disponibilité de la cible
    println!("cible en ligne ? : {}\n", target.is_up());

    println!("modif du is up en 'true'\n");

    target.set_is_up(true);
    println!("cible en ligne ? : {}", target.is_up());

    //ajout de quelque ports
    target.add_open_port(80);
    target.add_open_port(443);
    target.add_open_port(22);
    println!("ports 'ouverts' : {:?}", target.open_ports());
}
