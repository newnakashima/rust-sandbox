use std::io;
use std::cmp::Ordering;
use std::process::{exit};
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        guess = guess.trim().to_string();
        let guess_num: u32 = match guess.parse() {
            Ok(num) => num,
            Err(_) => {
                if guess == "quit" {
                    exit(0);
                }
                println!("{guess} is not a number.");
                continue;
            }
        };

        println!("You guessed: {guess_num}");

        match guess_num.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
