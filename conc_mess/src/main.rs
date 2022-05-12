use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let valeur = String::from("salut");
        tx.send(valeur).unwrap();
    });

    let recu = rx.recv().unwrap();
    println!("On a reçu : {}", recu);
}
