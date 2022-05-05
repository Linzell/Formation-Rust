use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
  let contenu = fs::read_to_string(config.nom_fichier)?;

  let resultats = if config.sensible_casse {
    rechercher(&config.recherche, &contenu)
  } else {
    rechercher_insensible_casse(&config.recherche, &contenu)
  };

  for ligne in resultats {
    println!("{}", ligne);
  }

  Ok(())
}

pub struct Config {
  pub recherche: String,
  pub nom_fichier: String,
  pub sensible_casse: bool,
}

impl Config {
  pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
    args.next();

    let recherche = match args.next() {
      Some(arg) => arg,
      None => return Err("nous n'avons pas de chaîne de caractères"),
    };

    let nom_fichier = match args.next() {
      Some(arg) => arg,
      None => return Err("nous n'avons pas de nom de fichier"),
    };

    let sensible_casse = env::var("MINIGREP_INSENSIBLE_CASSE").is_err();

    Ok(Config {
      recherche,
      nom_fichier,
      sensible_casse,
    })
  }
}

pub fn rechercher<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
  contenu
    .lines()
    .filter(|ligne| ligne.contains(recherche))
    .collect()
}

pub fn rechercher_insensible_casse<'a>(recherche: &str, contenu: &'a str) -> Vec<&'a str> {
  contenu
    .lines()
    .filter(|ligne| ligne.to_lowercase().contains(recherche))
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn sensible_casse() {
    let recherche = "duct";
    let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
Duck tape.";

    assert_eq!(
      vec!["sécurité, rapidité, productivité."],
      rechercher(recherche, contenu)
    );
  }

  #[test]
  fn insensible_casse() {
    let recherche = "rUsT";
    let contenu = "\
Rust:
sécurité, rapidité, productivité.
Obtenez les trois en même temps.
C'est pas rustique.";

    assert_eq!(
      vec!["Rust:", "C'est pas rustique."],
      rechercher_insensible_casse(recherche, contenu)
    );
  }
}
