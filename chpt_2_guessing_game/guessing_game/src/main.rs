use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn read_guess() -> Result<u32, std::num::ParseIntError> {
    println!("Please input your guess (between 1 and 100 inclusive):");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    return guess.trim().parse();
}

fn main() {
    let secret_number: u32 = rand::thread_rng().gen_range(1..101);
    let mut guesses: u8 = 10;
    println!("Welcome to the guessing game!");

    loop {
        let guess = match read_guess() {
            Ok(num) =>  num,
            Err(_) => continue,
        };
        println!("You guessed {}...", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => {
                guesses -= 1;
                println!("Too low ({} attempt(s) left)!", guesses);
                if guesses == 0 {
                    println!("Game over!");
                    break;
                }
            },
            Ordering::Greater => {
                guesses -= 1;
                println!("Too high ({} attempt(s) left)!", guesses);
                if guesses == 0 {
                    println!("Game over!");
                    break;
                }
            },
            Ordering::Equal => {
                println!("You got it!");
                break;
            },
        }
    }
}
