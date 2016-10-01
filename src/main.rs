extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    println!("Enter your guess:");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    
    loop {
        let mut guess = String::new();

        // println!("The secret number is: {}", secret_number);

        io::stdin().read_line(&mut guess)
        .expect("Failed to read input");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Higher!"),
            Ordering::Greater => println!("Lower!"),
            Ordering::Equal => {
                println!("You Win!");
                break;    
            }
        }
    }
}
