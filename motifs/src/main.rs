use std::fmt::Display;

fn main() {
  let couleur_favorite: Option<&str> = None;
  let on_est_mardi = false;
  let age: Result<u8, _> = "34".parse();

  if let Some(couleur) = couleur_favorite {
    println!(
      "Utilisation de votre couleur favorite, {}, comme couleur de fond",
      couleur
    );
  } else if on_est_mardi {
    println!("Mardi, c'est le jour du vert !");
  } else if let Ok(age) = age {
    if age > 30 {
      println!("Utilisation du violet comme couleur de fond");
    } else {
      println!("Utilisation de l'orange comme couleur de fond");
    }
  } else {
    println!("Utilisation du bleu comme couleur de fond");
  }

  let mut pile = Vec::new();

  pile.push(1);
  pile.push(2);
  pile.push(3);

  while let Some(donnee_du_haut) = pile.pop() {
    println!("{}", donnee_du_haut);
  }

  let v = vec!['a', 'b', 'c'];

  for (indice, valeur) in v.iter().enumerate() {
    println!("{} est à l'indice {}", valeur, indice);
  }

  let point = (3, 5);
  afficher_coordonnees(&point);

  let une_option_quelconque: Option<i32> = None;
  if let Some(x) = une_option_quelconque {
    println!("{}", x);
  }

  let x = 1;

  match x {
    1 => println!("un"),
    2 => println!("deux"),
    3 => println!("trois"),
    _ => println!("n'importe quoi"),
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("On a 50"),
    Some(y) => println!("Correspondance, y = {:?}", y),
    _ => println!("Cas par défaut, x = {:?}", x),
  }

  println!("A la fin : x = {:?}, y = {:?}", x, y);

  let x = 1;

  match x {
    1 | 2 => println!("un ou deux"),
    3 => println!("trois"),
    _ => println!("quelque chose d'autre"),
  }

  let x = 5;

  match x {
    1..=5 => println!("de un à cinq"),
    _ => println!("quelque chose d'autre"),
  }

  let x = 'c';

  match x {
    'a'..='j' => println!("lettre ASCII du début"),
    'k'..='z' => println!("lettre ASCII de la fin"),
    _ => println!("autre chose"),
  }

  let p = Point { x: 0, y: 7 };

  let Point { x: a, y: b } = p;
  println!("C'est {:?}", assert_eq!(0, a));
  println!("C'est {:?}", assert_eq!(7, b));

  let p = Point { x: 0, y: 7 };

  match p {
    Point { x, y: 0 } => println!("Sur l'axe x à la position {}", x),
    Point { x: 0, y } => println!("Sur l'axe y à la position {}", y),
    Point { x, y } => println!("Sur aucun des axes : ({}, {})", x, y),
  }

  let msg = Message::ChangerCouleur(Couleur::Tsv(0, 160, 255));

  match msg {
    Message::Quitter => {
      println!("La variante Quitter n'a pas de données à déstructurer.")
    }
    Message::Deplacer { x, y } => {
      println!("Déplacement de {} sur l'axe x et de {} sur l'axe y", x, y);
    }
    Message::Ecrire(text) => println!("Message textuel : {}", text),
    Message::ChangerCouleur(Couleur::Rvb(r, v, b)) => println!(
      "Changement des taux de rouge à {}, de vert à {} et de bleu à {}",
      r, v, b
    ),
    Message::ChangerCouleur(Couleur::Tsv(t, s, v)) => println!(
      "Changement des taux de teinte à {}, de saturation à {} et de valeur à {}",
      t, s, v
    ),
    _ => (),
  }
  let mut valeur_du_reglage = Some(5);
  let nouvelle_valeur_du_reglage = Some(10);

  match (valeur_du_reglage, nouvelle_valeur_du_reglage) {
    (Some(_), Some(_)) => {
      println!("Vous ne pouvez pas écraser une valeur déjà existante");
    }
    _ => {
      valeur_du_reglage = nouvelle_valeur_du_reglage;
    }
  }

  println!("Le réglage vaut {:?}", valeur_du_reglage);

  let nombres = (2, 4, 8, 16, 32);

  match nombres {
    (premier, _, troisieme, _, cinquieme) => {
      println!(
        "Voici quelques nombres : {}, {}, {}",
        premier, troisieme, cinquieme
      )
    }
  }
  let s = Some(String::from("Salutations !"));

  if let Some(_) = s {
    println!("j'ai trouvé une chaine de caractères");
  }

  println!("{:?}", s);

  let nombres = (2, 4, 8, 16, 32);

  match nombres {
    (premier, .., dernier) => {
      println!("Voici quelques nombres : {}, {}", premier, dernier);
    }
  }

  let nombre = Some(4);

  match nombre {
    Some(x) if x % 2 == 0 => println!("Le nombre {} est pair", x),
    Some(x) => println!("Le nombre {} est impair", x),
    None => (),
  }

  let x = Some(5);
  let y = 10;

  match x {
    Some(50) => println!("Nous obtenons 50"),
    Some(n) if n == y => println!("Nous avons une correspondance, n = {}", n),
    _ => println!("Cas par défaut, x = {:?}", x),
  }

  println!("Au final : x = {:?}, y = {}", x, y);

  let x = 4;
  let y = false;

  match x {
    4 | 5 | 6 if y => println!("yes"),
    _ => println!("no"),
  }

  enum Messages {
    Hello { id: i32 },
  }

  let msg = Messages::Hello { id: 5 };

  match msg {
    Messages::Hello {
      id: id_variable @ 3..=7,
    } => println!(
      "Nous avons trouvé un id dans l'intervalle : {}",
      id_variable
    ),
    Messages::Hello { id: 10..=12 } => {
      println!("Nous avons trouvé un id dans un autre intervalle")
    }
    Messages::Hello { id } => println!("Nous avons trouvé un autre id : {}", id),
  }
}

fn afficher_coordonnees(&(x, y): &(i32, i32)) {
  println!("Coordonnées actuelles : ({}, {})", x, y);
}

struct Point {
  x: i32,
  y: i32,
}

enum Message {
  Quitter,
  Deplacer { x: i32, y: i32 },
  Ecrire(String),
  ChangerCouleur(Couleur),
}

enum Couleur {
  Rvb(i32, i32, i32),
  Tsv(i32, i32, i32),
}
