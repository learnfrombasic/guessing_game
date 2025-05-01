use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn generate_secret_number() -> u32 {
    let num: u32 = rand::thread_rng().gen_range(1..=100);
    num
}

fn comparing(secret_number: u32, guess: u32) -> bool {
    match secret_number.cmp(&guess) {
        Ordering::Less => {println!("Too small!"); false},
        Ordering::Greater => {println!("Too big!"); false},
        Ordering::Equal => {println!("You win!"); true},
    }
}

fn main() {
    /*
    - `mut` keyword make it mutable ~ changable. 
    */
    let mut n_trial :u32 = 5; // you can remove this to make the game unlimited
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        /*
        Rust's variable is immutable by default. 
        assign it with `mu` make it mutable ~ changable. 
        */
        let mut guess = String::new(); 

        /*
        Calling .read_line() from io::stdin to read 
        user's input. 
        The code also read a reference of a mutable variable, guess, so that it 
        access one piece of data without needing to copy that data 
        into memory multiple times.
        */
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        /*
        - `trim` method on a String instance will eliminate any whitespace at the beginning and end
        - `parse` method will convert a string into a number
        */
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if comparing(generate_secret_number(), guess) && n_trial > 0 {
            break;
        }
        n_trial -= 1;
    }

    println!("You lose!");
}

