// crates are a way to package and share code with others
// crates.io is the official Rust community's crate registry

// add the rand crate to the dependencies in Cargo.toml
// [dependencies]
// rand = "0.8.5"

// use the rand crate to generate a random number
// use the io module from the standard library to read input from the user
// use the cmp module from the standard library to compare the guess to the secret number
use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // create a mutable String to store the user's guess
    let mut guess = String::new();

    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        // clear the previous guess
        guess.clear();

        // read a line from the standard input
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the input to a number
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a valid number!");
                continue;
            }
        };

        println!("You guessed: {}", guess);

        // compare the guess to the secret number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
