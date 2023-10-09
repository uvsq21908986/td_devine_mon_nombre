use std::io;
use rand::Rng;  // Assurez-vous d'avoir cet import

fn main() {
    println!("Devine mon nombre !");
    println!("Saisissez votre proposition.");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).unwrap();
    println!("Vous avez saisi : {}", guess);

    // Cette ligne est maintenant à l'intérieur de la fonction main
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("Le nombre secret est : {}", secret_number);
}
