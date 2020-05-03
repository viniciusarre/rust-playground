use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main () {
    loop {
        println!("Guess the number!");
        println!("Please input your guess.");
        println!("Input \"\\q\", \"quit\", or \"exit\" to exit");
        let secret_number = rand::thread_rng().gen_range(1, 101);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
       
        if should_quit(guess.trim()) {
            break;
        }
        let guess = match guess.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => {
                println!("Only numbers are accepted... starting over... {}", guess);
                continue;
            }
        };
        println!("You guessed {}", guess);
        println!("Secret number is: {} ", secret_number);
        let guess_result = order_guess(guess, secret_number);
        println!("{}", guess_result);
        if guess_result == "You win" {
            break;
        }
    }
}

fn should_quit(v: &str) -> bool {
    match v.trim().to_lowercase().as_str() {
        "quit" | "\\q" | "exit" => {
            println!("Gracefully exiting!");
            true
        }
        _ => false
    }
}

fn order_guess(guess: u32, secret_number: u32) -> String {
    match guess.cmp(&secret_number) {
        Ordering::Less => String::from("Too Small"),
        Ordering::Greater => String::from("Too big!"),
        Ordering::Equal => String::from("You win")
    }
}