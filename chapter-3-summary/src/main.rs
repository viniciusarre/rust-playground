use std::io;
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

fn fahrenheit_to_celsius(celsius_temp: f64) -> f64{
    celsius_temp * (9.0 / 5.0) + 32.0
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
    println!("Second option chosen!");
}

fn third_option() {
    println!("Third option chosen!");
}
