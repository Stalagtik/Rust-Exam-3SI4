//Importer clap pour gérer les arguments de la ligne de commande
use clap::{App, Arg};
//Importer le module scanner et la fonction scan_port
mod scanner;
use scanner::scan_range_port;

/// Fonction principale
/// Cette fonction gère les arguments de la ligne de commande et appelle la fonction scan_range_port pour scanner les ports spécifiés sur l'adresse IP donnée.
/// Les paramètres attendus sont l'adresse IP et les ports à scanner.
fn main() {
    //Définir les arguments de la ligne de commande et les aides
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("ESGI")
        .about("Scanne une plage de ports spécifique sur une adresse IP donnée")
        .arg(Arg::new("ip")
            .help("Adresse IP à scanner")
            .required(true)
            .index(1))
        .arg(Arg::new("ports")
            .help("Ports à scanner, peut être une liste séparée par des virgules ou une plage avec un tiret")
            .required(true)
            .index(2))
        .arg(Arg::new("debug")
            .short('d')
            .long("debug")
            .help("Affiche tous les ports, ouverts et fermés")
            .takes_value(false))
        .get_matches();

    //Récupérer les valeurs des arguments
    let ip = matches.value_of("ip").unwrap();
    let ports_str = matches.value_of("ports").unwrap();
    let show_all = matches.is_present("debug");

    //Parser les ports
    let ports = match scanner::parse_ports(ports_str) {
        Ok(ports) => ports,
        Err(e) => {
            eprintln!("Erreur d'analyse des ports: {}", e);
            return;
        }
    };

    //Appeler la fonction scan_range_port et afficher les résultats
    let results = scan_range_port(ip, &ports);

    //Afficher les résultats
    for (port, is_open) in results {
        if is_open {
            println!("Le port {} sur {} est ouvert.", port, ip);
        } else if show_all {
            println!("Le port {} sur {} est fermé.", port, ip);
        }
    }
}
