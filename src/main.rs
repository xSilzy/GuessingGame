use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {


    let secret_number = rand::thread_rng().gen_range(1..=100); // Generating the secret number
    let repetition_count = 6;
    let mut guessed_correctly = false;

    println!("Guess the number!"); // Game Title

    // Game Loop
    for i in 0..repetition_count {
        let mut guess = String::new();

        // User-input prompting and handling
        println!("Please input your guess..");
        println!("Guesses Left: {}/{repetition_count}",repetition_count - i);
        println!();

        io::stdin() // Read user-input
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() { // Convert user-input to integer
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number.");
                continue;
            }
        };

        // Compare the user-input to the randomly generated number
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Correct! You win!");
                guessed_correctly = true;
                break;
            }
        }
    }
    // Send fail message if number couldn't be guessed
    if !guessed_correctly{
        println!("Out of guesses! Try again! \nThe correct number was {secret_number}");
    }
}