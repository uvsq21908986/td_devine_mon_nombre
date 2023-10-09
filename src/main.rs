use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Devine mon nombre !");
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Saisissez votre proposition.");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).unwrap();

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Veuillez saisir un nombre valide !");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Trop petit !"),
            Ordering::Equal => {
                println!("Bravo ! Vous avez deviné le nombre.");
                break;
            },
            Ordering::Greater => println!("Trop grand !"),
        }
    }
}
