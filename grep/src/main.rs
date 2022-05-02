use std::env;
use std::process;

use libs::Config;

fn main() {
  let args: Vec<String> = env::args().collect();

  let config = Config::new(&args).unwrap_or_else(|err| {
    println!(
      "Problème rencontré lors de l'interprétation des arguments : {}",
      err
    );
    process::exit(1);
  });

  println!("On recherche : {}", config.recherche);
  println!("Dans le fichier : {}", config.nom_fichier);

  if let Err(e) = libs::run(config) {
    println!("Erreur applicative : {}", e);

    process::exit(1);
  }
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
  vec![]
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn un_resultat() {
    let recherche = "duct";
    let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.";

    assert_eq!(
      vec!["sécurité, rapidité, productivité."],
      rechercher(recherche, contenu)
    );
  }
}
