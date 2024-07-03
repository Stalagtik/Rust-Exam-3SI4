//Importer clap pour gérer les arguments de la ligne de commande
use clap::{App, Arg};
//Importer le module scanner et la fonction scan_port
mod scanner;
use scanner::scan_port;

/// Fonction principale
/// Cette fonction gère les arguments de la ligne de commande et appelle la fonction scan_port pour scanner le port spécifié sur l'adresse IP donnée.
/// Les paramètres attendus sont l'adresse IP et le port à scanner.
fn main() {
    //Définir les arguments de la ligne de commande et les aides
    let matches = App::new("Port Scanner")
        .version("1.0")
        .author("ESGI")
        .about("Scanne un port spécifique sur une adresse IP donnée")
        .arg(Arg::new("ip")
            .help("Adresse IP à scanner")
            .required(true)
            .index(1))
        .arg(Arg::new("port")
            .help("Port à scanner")
            .required(true)
            .index(2))
        .get_matches();

    //Récupérer les valeurs des arguments
    let ip = matches.value_of("ip").unwrap();
    let port: u16 = matches.value_of_t("port").unwrap_or_else(|e| e.exit());

    //Appeler la fonction scan_port et afficher le résultat
    let is_open = scan_port(ip, port);

    if is_open {
        //Afficher un message si le port est ouvert
        println!("Le port {} sur {} est ouvert.", port, ip);
    } else {
        //Afficher un message si le port est fermé
        println!("Le port {} sur {} est fermé.", port, ip);
    }
}