use std::io;
use std::cmp::Ordering;
use std::num::ParseIntError;
use rand::Rng;

pub struct Guess {
    value: i32,
}

impl Guess {
    fn validate(input: &str) -> Result<i32, ParseIntError> {
        input.trim().parse()
    }

    pub fn new(value: &str) -> Guess {
        let value = match Guess::validate(value) {
            Ok(v) => v,
            Err(_) => panic!("{} is not a number. Please input a number", value.trim()),
        };

        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // guess = guess.trim().to_string();
        // let guess_num: u32 = match guess.parse() {
        //     Ok(num) => num,
        //     Err(_) => {
        //         if guess == "quit" {
        //             exit(0);
        //         }
        //         println!("{guess} is not a number.");
        //         continue;
        //     }
        // };
        let guess = Guess::new(&guess);
        let guess_num = guess.value;

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
