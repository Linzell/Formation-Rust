use ajouter_un;
use ajouter_deux;
use rand;

fn main() {
    let nombre = rand::random::<i32>();
    println!(
        "Hello, world ! {} plus un vaut {} !",
        nombre,
        ajouter_un::ajouter_un(nombre)
    );

    let nombre = rand::random::<i32>();
    println!(
        "Hello, world ! {} plus deux vaut {} !",
        nombre,
        ajouter_deux::ajouter_deux(nombre)
    );

}