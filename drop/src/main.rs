struct PointeurPerso {
  donnee: String,
}

impl Drop for PointeurPerso {
  fn drop(&mut self) {
    println!(
      "Nettoyage d'un PointeurPerso avec la donnée `{}` !",
      self.donnee
    );
  }
}

fn main() {
  let c = PointeurPerso {
    donnee: String::from("des trucs"),
  };
  println!("PointeurPerso créé.");
  drop(c);
  let d = PointeurPerso {
    donnee: String::from("d'autres trucs"),
  };
  println!("PointeurPerso libéré avant la fin du main.");
}
