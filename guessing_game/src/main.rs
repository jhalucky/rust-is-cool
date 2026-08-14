use std::io;
use rand::Rng;
use std::cmp::Ordering;


fn main() {
    println!("Guess any number between 1 and 20");

    let secret_number = rand::thread_rng().gen_range(1..=20);
    // println!("The secret number is: {secret_number}"); Win game by commenting this out
    println!("Enter your guess:");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Enter a guess");

    let guessed_number: i32 = guess.trim().parse().expect("Guessed Number");

    println!("You guessed: {guessed_number}");

    match guessed_number.cmp(&secret_number) {
        Ordering::Less => println!("Too less!"),
        Ordering::Greater => println!("Too greater"),
        Ordering::Equal => println!("Right Guess!!!")
    }

}
