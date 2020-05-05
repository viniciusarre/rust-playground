use std::io;
extern crate num_bigint;
extern crate num_traits;

use num_bigint::BigUint;
// use num_bigint::ToBigUint;
use num_traits::{Zero, One};

fn main() {
    loop {
        println!("Input a number up to 10,000");
        let mut number = String::new();
        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line");
        let number: usize = match number.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Please input a valid number");
                0
            }
        };

        if number > 10_000 {
            println!("Number too big, please, try again!");
            continue;
        } else if number == 0 {
            continue;
        }

        

        let factorial = factorial(number);
        println!("Factorial of {} is: {} ", number, factorial);
        // let factorial = loop_factorial(number);
        // println!("[LOOP] Factorial of {} is: {} ", number, factorial);
    }
   
}

fn factorial (n: usize) -> BigUint {
    if n == Zero::zero() {
        One::one()
    } else {
        factorial(n - 1) * n
    }  
}

// Another solution (non-recursive):
// fn loop_factorial (n: usize) -> BigUint {
//     let mut result :BigUint = One::one();
//     for i in (1..n).rev() {
//         result *= i;
//     }
//     result
// }