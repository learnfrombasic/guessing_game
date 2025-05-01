# 🎯 Guessing Game in Rust (n-Trial Version)

A beginner-friendly number guessing game written in Rust — but with a twist: you only get **n-tries**, and the secret number changes each round!

Built with:
- Standard input/output
- Basic pattern matching and error handling
- Random number generation using the `rand` crate

---

## 📦 Features

- You get **n trials** to guess a number between 1 and 100.
- A **new secret number** is generated on every trial.
- Tells you if your guess is too high, too low, or correct.
- Ends the game early if you guess correctly.
- Lightweight, beginner-safe Rust app with comments included.

---

## 🛠 Installation

### Prerequisites
- Rust toolchain: [Install Rust](https://rustup.rs/)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 🚀 Run the Game

```bash
# Create a new project if you haven't
cargo new guessing_game
cd guessing_game

# Replace the contents of src/main.rs with your code

# Add rand crate to Cargo.toml
cargo add rand

# Run the game
cargo run
```

---

## 📄 Example Output

```text
Guess the number!
Please input your guess.
42
You guessed: 42
Too big!

Please input your guess.
19
You guessed: 19
Too small!

...

You lose!
```

---

## 📁 Code Overview (`src/main.rs`)

```rust
use std::io;
use std::cmp::Ordering;
use rand::Rng;

// Generate a new secret number between 1 and 100
fn generate_secret_number() -> u32 {
    rand::thread_rng().gen_range(1..=100)
}

// Compare the secret number with the user's guess
fn comparing(secret_number: u32, guess: u32) -> bool {
    match secret_number.cmp(&guess) {
        Ordering::Less => {println!("Too small!"); false},
        Ordering::Greater => {println!("Too big!"); false},
        Ordering::Equal => {println!("You win!"); true},
    }
}

fn main() {
    let mut n_trial: u32 = 5;   // you can remove this to make the game unlimited
    println!("Guess the number!");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        println!("You guessed: {}", guess);

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        // Check the guess
        if comparing(generate_secret_number(), guess) && n_trial > 0 {
            break;
        }

        n_trial -= 1;
        if n_trial == 0 {
            println!("You lose!");
            break;
        }
    }
}
```

---

## 🧠 Learning Highlights

- `rand::thread_rng().gen_range(...)` for randomness
- `match` + `Ordering` enum for comparison
- Handling mutable state with `mut`
- Working with user input using `io::stdin()`
