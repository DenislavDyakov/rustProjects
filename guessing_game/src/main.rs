use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Welcome to the guess the number game!");

    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);

    println!("Please input your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let  guess: u32 = guess.trim().parse().expect("Please type a number!");

    println!("You guessed: {guess}");
    println!("The secret number is: {secret_number}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your guess is too small!"),
        Ordering::Greater => println!("Your guess is too big!"),
        Ordering::Equal => println!("You win!"),
    }

    println!("Goodbye!")
}
