use std::ops::Add;

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
  x: i32,
  y: i32,
}

impl Add for Point {
  type Output = Point;

  fn add(self, other: Point) -> Point {
    Point {
      x: self.x + other.x,
      y: self.y + other.y,
    }
  }
}

trait Pilote {
  fn voler(&self);
}

trait Magicien {
  fn voler(&self);
}

struct Humain;

impl Pilote for Humain {
  fn voler(&self) {
    println!("Ici le capitaine qui vous parle.");
  }
}

impl Magicien for Humain {
  fn voler(&self) {
    println!("Décollage !");
  }
}

impl Humain {
  fn voler(&self) {
    println!("*agite frénétiquement ses bras*");
  }
}

fn mesure(premier: Point, deuxieme: Point, resultat: Point) {
  assert_eq!(premier + deuxieme, resultat);
}

trait Animal {
  fn nom_bebe() -> String;
}

struct Chien;

impl Chien {
  fn nom_bebe() -> String {
      String::from("Spot")
  }
}

impl Animal for Chien {
  fn nom_bebe() -> String {
      String::from("chiot")
  }
}

use std::fmt;

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        let valeur = self.to_string();
        let largeur = valeur.len();
        println!("{}", "*".repeat(largeur + 4));
        println!("*{}*", " ".repeat(largeur + 2));
        println!("* {} *", valeur);
        println!("*{}*", " ".repeat(largeur + 2));
        println!("{}", "*".repeat(largeur + 4));
    }
}

impl OutlinePrint for Point {}


impl fmt::Display for Point {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "({}, {})", self.x, self.y)
  }
}

struct Enveloppe(Vec<String>);

impl fmt::Display for Enveloppe {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

fn main() {
  mesure(
    Point { x: 2, y: 0 },
    Point { x: 1, y: 3 },
    Point { x: 3, y: 3 },
  );
  mesure(
    Point { x: 5, y: 2 },
    Point { x: 0, y: 11 },
    Point { x: 5, y: 13 },
  );

  let une_personne = Humain;
  Pilote::voler(&une_personne);
  Magicien::voler(&une_personne);
  une_personne.voler();

  println!("Un bébé chien s'appelle un {}", <Chien as Animal>::nom_bebe());

  let p = Point { x: 1, y: 3 };
  p.outline_print();

  let w = Enveloppe(vec![String::from("hello"), String::from("world")]);
  println!("w = {}", w);

}
