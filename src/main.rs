// use core::num;
use std::io;
use std::cmp::Ordering;
use rand::Rng;
use colored::*;

fn main() {
    println!("---- Guessing game! ----");

    let secret_number = rand::thread_rng().gen_range(1,101);

    // println!("Secret Number: {}", secret_number);
    
    let mut tries: u32 = 0;

    loop {
        println!("Guess the number: ");
    
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
        println!("You guessed: {}", guess);
        tries += 1;
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}", "Too small!".red()),
            Ordering::Greater => println!("{}", "Too big!".red()),
            Ordering::Equal => {
                println!("{}","You win!".green());
                println!("You tried {} times", tries);
                break;
            },
        }
    }
 
}
