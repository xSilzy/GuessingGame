use std::io;
use rand::Rng;

fn main() {
    // initialize variables
    const MIN_RANGE: i32 = 1;
    const MAX_RANGE: i32 = 100;
    const MAX_GUESSES: i32 = 6;

    'game_loop: loop {
        let random_number: i32 = rand::thread_rng().gen_range(MIN_RANGE..=MAX_RANGE);
        let percent_25: f32 = (random_number as f32/MAX_RANGE as f32) * 25.0; // 25%
        let percent_5: f32 = (random_number as f32/MAX_RANGE as f32) * 5.0; // 5%
        let mut guessed_correctly = false;
        let mut result_log: Vec<String> = Vec::new();

        println!("!!!Worlde inspired number guessing game!!!");
        println!("⬆️/⬇️🟥️ means you've guessed more than 25% away from the correct number");
        println!("⬆️/⬇️🟨️ means you've guessed within 25% of the correct number");
        println!("You win if you guess within 5% of the correct number");


        println!("Please enter a number... \n");

        for current_guess in 1..=MAX_GUESSES{

            println!("----");
            println!("{}/{MAX_GUESSES} Guesses Left\n", MAX_GUESSES - current_guess + 1); // +1 to bring it back up to 6/6 at the start

            let mut guess = String::new();

            io::stdin().read_line(&mut guess).expect("Failed to read line");

            let guess: i32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("Please input a number!");
                    continue;
                }
            };

            println!();

            let diff: f32 = random_number as f32 - guess as f32;

            if diff.abs() <= percent_5 {

                guessed_correctly = true;
                println!("Correct!");
                println!("The number was {random_number}!");

                let format = format!("---\nGuesses left: {}/{MAX_GUESSES}", MAX_GUESSES - current_guess + 1 );
                result_log.insert(0, format);
                result_log.push("✅".to_string());

                break

            } else if diff.abs() <= percent_25 && diff.abs() > percent_5 {

                let arrow = if diff > 0.0 {"⬆️"} else {"⬇️"};
                println!("{arrow}🟨️");

                let format = format!("{arrow}🟨️");
                result_log.push(format);

            } else if diff.abs() > percent_25 {

                let arrow = if diff > 0.0 {"⬆️"} else {"⬇️"};
                println!("{arrow}️🟥️");

                let format = format!("{arrow}️🟥️");
                result_log.push(format);
            }
        }

        if !guessed_correctly {
            println!("The number was {random_number}!");
            let format = format!("---\nGuesses left: X/{MAX_GUESSES}");
            result_log.insert(0, format);
        }

        for line in result_log {
            println!("{}", line);
        }

        loop {
            print!("Type 'quit' to exit or 'restart' to restart: ");
            io::Write::flush(&mut io::stdout()).unwrap();
            let mut command_input = String::new();
            io::stdin().read_line(&mut command_input).expect("Failed to read line");

            let command = command_input.trim().to_lowercase();

            if command == "quit" {
                // Breaks out of the labeled 'app_loop' (exiting the program)
                println!("Exiting.");
                break 'game_loop;
            } else if command == "restart" {
                println!("Restarting...");
                break;
            } else {
                // Invalid command. The inner loop automatically runs again.
                println!("Invalid command. Please type 'quit' or 'restart'.");
            }
        }
    }
}
