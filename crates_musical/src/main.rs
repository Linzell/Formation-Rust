use libs::CouleurPrimaire;
use libs::mixer;

fn main() {
    let rouge = CouleurPrimaire::Bleu;
    let jaune = CouleurPrimaire::Jaune;
    let resultat = mixer(rouge, jaune);

    println!("{:?}", resultat);
}