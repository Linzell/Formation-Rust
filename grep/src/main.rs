use std::env;
use std::process;

use libs::Config;

fn main() {
  let config = Config::new(env::args()).unwrap_or_else(|err| {
    eprintln!(
      "Problème rencontré lors de l'interprétation des arguments : {}",
      err
    );
    process::exit(1);
  });

  if let Err(e) = libs::run(config) {
    eprintln!("Erreur applicative : {}", e);

    process::exit(1);
  }
}
