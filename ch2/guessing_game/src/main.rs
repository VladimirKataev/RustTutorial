use std::io; // similar to namespaces 
use std::cmp::Ordering;
use rand::prelude::*;
fn main() {
    let sercret_number = rand::rng().random_range(1..=100);
    println!("secret number is {}", sercret_number);
    println!("Guess the number");

    loop{
        let mut guess = String::new();
        println!("Enter your guess.");
        io::stdin()                         // library
            .read_line(&mut guess)          // write into guess
            .expect("Failed to read line"); // read_line returns result, which 
                                            // might be of enum Error. 
                                            // If it is, panic!

        let guess : u32 = 
            match guess.trim().parse(){
                Ok(num) => num,
                Err(_) => continue, //jump back to start of loop
            };
        println!("You guessed: {guess}");

        match guess.cmp(&sercret_number){
            Ordering::Less => println!("Guess higher"),
            Ordering::Equal => {
                println!("Correct");
                break;
            },
            Ordering::Greater => println!("Guess lower"),
        }
    }
}
