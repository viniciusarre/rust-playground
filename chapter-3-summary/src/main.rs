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
        _ => fibonacci_nth(n - 1) + fibonacci_nth(n - 2),
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
    print_lyrics();
}

fn get_gift_by_day (day: u8) -> String { 
    match day {
        1 => String::from("A Partridge in a Pear Tree"),
        2 => String::from("Two Turtle Doves"),
        3 => String::from("Three French Hens"),
        4 => String::from("Four Calling Birds"),
        5 => String::from("Five Golden Rings"),
        6 => String::from("Six Geese a Laying"),
        7 => String::from("Seven Swans a Swimming"),
        8 => String::from("Eight Maids a Milking"),
        9 => String::from("Nine Ladies Dancing"),
        10 => String::from("Ten Lords a Leaping"),
        11 => String::from("Eleven Pipers Piping"),
        12 => String::from("Twelve Drummers Drumming"),
        _ => String::from("Not found!")
    }

}

fn get_lyrics_by_index (day: u8) -> String {
    let mut lyrics = String::with_capacity(500);
    if day == 1 {
        lyrics.push_str("\n");
        lyrics.push_str(&get_gift_by_day(day));
    } else {
        for i in (1..day + 1).rev() {
            if i > 1 {
                let gift = get_gift_by_day(i );
                let line_break = "\n";
                lyrics.push_str(line_break);
                lyrics.push_str(&gift);
            } else {
                let gift = get_gift_by_day(i);
                let line_break = "\nAnd ";
                lyrics.push_str(line_break);
                lyrics.push_str(&gift);   
            }
        }
    }
    lyrics
}

fn print_lyrics () {
    for i in 1..13 {
        let beginning = format!("{} {} {} ", "On the", get_day(i), "of Christmas my true love sent me:");;
        let lyrics = get_lyrics_by_index(i);
        println!("{} {} \n", beginning, lyrics)

    }
}

fn get_day(index: u8) -> String {
    match index {
        1 => String::from("first"),
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("fourth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eighth"),
        9 => String::from("nineth"),
        10 => String::from("tenth"),
        11 => String::from("eleventh"),
        12 => String::from("twelfth"),
        _ => String::from("")
    }
}
