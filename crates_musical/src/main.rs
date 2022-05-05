use libs::CouleurPrimaire;
use libs::mixer;

fn main() {
    let rouge = CouleurPrimaire::Rouge;
    let jaune = CouleurPrimaire::Jaune;
    mixer(rouge, jaune);
}