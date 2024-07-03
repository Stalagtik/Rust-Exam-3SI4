mod fonctions;

use clap::{App, Arg};
use crate::fonctions::{host_is_up, check_ip_range};

fn main() {
    // Configuration de l'interface en utilisant clap
    let matches = App::new("Host Checker")
        .version("1.0")
        .author("Theo/Jiullian")
        .about("Vérifie si un hôte est en ligne ou non")
        .arg(Arg::new("address")
            .help("Adresse IP ou FQDN de l'hôte à vérifier. Pour vérifier une plage d'adresses, utilisez CIDR (par exemple, X.X.X.X/24)")
            .required(true)
            .index(1))
        .get_matches();

    // Récupération de l'entrée utilisateur
    let address = matches.value_of("address").unwrap();

    // Vérification si l'adresse est une plage CIDR
    if address.contains('/') {
        check_ip_range(address);
    } else {
        // Vérification de l'adresse individuelle
        let is_up = host_is_up(address);

        if is_up {
            println!("L'adresse : {} est en ligne", address);
        } else {
            println!("L'adresse : {} est pas en ligne", address);
        }
    }
}
