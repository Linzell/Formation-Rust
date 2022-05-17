fn ajouter_un(x: i32) -> i32 {
  x + 1
}

fn le_faire_deux_fois(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

fn main() {
  
  let reponse = le_faire_deux_fois(ajouter_un, 5);

  println!("La réponse est : {}", reponse);

  let liste_de_nombres = vec![1, 2, 3];
  let liste_de_chaines: Vec<String> = liste_de_nombres.iter().map(|i| i.to_string()).collect();

  println!("La liste de chaines est : {:?}", liste_de_chaines);

  let liste_de_nombres = vec![4, 5, 6];
  let liste_de_chaines: Vec<String> = liste_de_nombres.iter().map(ToString::to_string).collect();

  println!("La liste de chaines est : {:?}", liste_de_chaines);

}
