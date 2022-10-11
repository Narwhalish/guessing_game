use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    const QUIT_PHRASE: &str = "quit";

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("Guess the number!");
    // println!("[DEBUG] Secret number {secret_number}");

    loop {
        println!("Input your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        guess = String::from(guess.trim());

        match guess.cmp(&String::from(QUIT_PHRASE)) {
            Ordering::Equal => {
                println!("Quitting game...");
                break;
            }
            _ => {}
        }

        let guess: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number!");
                continue;
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        println!("You guessed {guess}.");
    }
}
