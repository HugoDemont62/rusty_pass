use rand::Rng;
use rand::distributions::Alphanumeric;
use std::iter;

fn main() {
    let mut rng = rand::thread_rng();
    let password: String = iter::repeat(())
        .map(|()| rng.sample(Alphanumeric))
        .map(char::from)
        .take(30) // vous pouvez choisir la longueur du mot de passe ici
        .collect();

    println!("Mot de passe généré : {}", password);
}