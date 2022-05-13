use libs::Affichable;

struct ListeDeroulante {
  largeur: u32,
  hauteur: u32,
  options: Vec<String>,
}

impl Affichable for ListeDeroulante {
  fn afficher(&self) {
    println!(
      "Liste déroulante de taille {}x{}",
      self.largeur, self.hauteur
    );

    for option in self.options.iter() {
      println!("{}", option);
    }
  }
}

use libs::{Bouton, Ecran};

fn main() {
  let ecran = Ecran {
    composants: vec![
      Box::new(ListeDeroulante {
        largeur: 75,
        hauteur: 10,
        options: vec![
          String::from("Oui"),
          String::from("Peut-être"),
          String::from("Non"),
        ],
      }),
      Box::new(Bouton {
        largeur: 50,
        hauteur: 10,
        libelle: String::from("OK"),
      }),
    ],
  };

  ecran.executer();
}
