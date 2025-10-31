use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {

    let mut guess = String::new(); // mutable string
    let secret = rand::thread_rng().gen_range(1..=100);
    println!("Guess the number (Not {}):", secret);

    io::stdin()
        .read_line(&mut guess) // read into a mutable reference
        .expect("Failed to read line"); // deal with result potentially being an Err
    
    println!("You guessed {guess}");
    match guess.cmp(&secret){
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("Perfect"),
    }
}
