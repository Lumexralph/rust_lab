use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut input_attempt = 5;

    loop {
        // check the game attempt
        if input_attempt == 0 {
            println!("Oops! You are out of luck");
            println!("The secret number is {}, better luck next time", secret_number);
            return;
        }
        println!("Hi! you have {} attempt(s)", input_attempt);
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // It results the enum Result that needs to be handled
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
        input_attempt = input_attempt - 1;
    }
}
