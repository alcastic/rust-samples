use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("=== Guess the number ===");

    let _secret_number = rand::thread_rng().gen_range(1, 11);

    println!("Please input your guess (from 1 to 10):");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Input is not a number");

    match guess.cmp(&_secret_number) {
        Ordering::Less => println!("Your guess: {} is less tha secret_number", guess),
        Ordering::Equal => println!("Your guess: {} is less tha secret_number", guess),
        Ordering::Greater => println!("Your guess: {} is less tha secret_number", guess),
    }
}
