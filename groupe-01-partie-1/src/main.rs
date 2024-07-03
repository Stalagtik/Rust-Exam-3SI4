mod fonctions;

use clap::{App, Arg};
use crate::fonctions::host_is_up;

fn main() {
    //configuration de l'interface
    let matches = App::new("Host Checker")
        .version("1.0")
        .author("Theo/Jiullian")
        .about("verifie si un host est en ligne ou non")
        .arg(Arg::new("address")
            .help("adresse IP ou FQDN de l'hôte à vérifier")
            .required(true)
            .index(1))
        .get_matches();

    //récupére l'entrée du user
    let address = matches.value_of("address").unwrap();

    let is_up = host_is_up(address);

    if is_up {
        println!("l'adresse : {} est en ligne", address);
    } else {
        println!("l'adresse : {} est pas en ligne", address);
    }
}
