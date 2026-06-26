use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number between 1 and 100!");
    let secrete_number = rand::thread_rng().gen_range(1..=100);
    println!("This is the secrete number: {secrete_number}");

    loop {
        println!("Please enter your guess:");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(numb) => numb,
            Err(_) => continue,
        };      

        println!("Your guess is {guess}");
        
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("go higher"),
            Ordering::Greater => println!("go lower"),
            Ordering::Equal => { 
                println!("You goddamn right");
                break; 
            },
        }
    } //loop end
} //main 
