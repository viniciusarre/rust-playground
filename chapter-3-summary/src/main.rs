use std::io;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn main() {
    loop {
        println!("****************************************************************");
        println!("**** Please, pick an option: **");
        println!("**** 1. Fahrenheit to Celsius converter **");
        println!("**** 2. Get the nth Fibonacci number **");
        println!("**** 3. Print the lyrics to \"The Twelve Days of Christmas\" **");
        println!("**** Enter \"q\", quit or \"exit\" to close the program **");
        println!("****************************************************************");
        let mut option = String::new();
        io::stdin()
            .read_line(&mut option)
            .expect("Failed to read line");
        match option.trim().to_lowercase().as_str() {
            "1" => {
                first_option();
            }

            "2" => {
                second_option();
            }

            "3" => {
                third_option();
            }

            "quit" | "q" | "exit" => {
                break;
            }

            _ => {
                print!("Invalid option!");
                continue;
            }
        }
    }
}

fn fahrenheit_to_celsius(celsius_temp: f64) -> f64 {
    celsius_temp * (9.0 / 5.0) + 32.0
}

fn fibonacci_nth(n: u32) -> BigUint {
    match n {
        0 => Zero::zero(),
        1 => One::one(),
        _ => fibonacci_nth(n - 1) + fibonacci_nth(n - 1),
    }
}

fn first_option()  {
    println!("Please, input a temperature in Celsius");
    let mut temperature = String::new();
    io::stdin()
        .read_line(&mut temperature)
        .expect("Failed to read line");
    let temperature: f64 = temperature.trim().parse()
        .expect("Invalid number!");
    let result = fahrenheit_to_celsius(temperature);
    println!("Result: { } ", result)
}

fn second_option() {
    println!("Please input a positive number");
    let mut number = String::new();
    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line");
    let number = number.trim().parse::<u32>()
                        .expect("Invalid number!");
    let result = fibonacci_nth(number);
    println!("The {}th Fibonacci number is: {} ", number, result);
}

fn third_option() {
    println!("Third option chosen!");
}
