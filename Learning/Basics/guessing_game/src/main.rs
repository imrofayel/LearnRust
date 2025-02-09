// Read Docs: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/* Another neat feature of Cargo is that running the cargo doc --open command will build documentation provided by all your dependencies locally and open it in your browser. */

fn main() {
    let number = rand::rng().random_range(1..=29);

    //  like variables, references are immutable by default. Hence, you need to write &mut guess rather than &guess to make it mutable.

    loop {
        println!("Guess a number!");

        // In Rust, variables are immutable by default, meaning once we give the variable a value, the value won’t change.

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Every pattern must be handled exhaustively either explicitly or by using wildcards like _ in the match.
        let parsed_number: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number.");
                continue;
            }
        };

        match parsed_number.cmp(&number) {

            Ordering::Equal => {
                println!("You win! {} is correct.", guess.trim());
                break;
            },
            Ordering::Greater => println!("{parsed_number} is greater."),
            Ordering::Less => println!("{parsed_number} is smaller."),
        }
    }
}
