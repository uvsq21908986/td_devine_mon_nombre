use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101); // Assurez-vous d'avoir importé la crate 'rand'

    loop {
        let input = read_int_from_stdin();

        if let Some(input) = input { // if_let
            let comparison = get_ordering(secret_number, input);
            display_result(comparison);

            if has_found(comparison) {
                break;
            }
        } else {
            println!("Saisie incorrecte");
        }
    }
}

fn read_int_from_stdin() -> Option<u32> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).ok()?;
    input.trim().parse().ok()
}

fn get_ordering(secret_number: u32, input: u32) -> Ordering {
    secret_number.cmp(&input)
}

fn display_result(comparison: Ordering) {
    match comparison {
        Ordering::Equal => println!("Félicitations! Vous avez trouvé le bon nombre."),
        Ordering::Less => println!("Trop grand! Essayez un nombre plus petit."),
        Ordering::Greater => println!("Trop petit! Essayez un nombre plus grand."),
    }
}

fn has_found(comparison: Ordering) -> bool {
    comparison == Ordering::Equal
}
