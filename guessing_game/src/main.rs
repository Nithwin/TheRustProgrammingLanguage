use std::io;
use std::cmp::Ordering;
use rand::Rng;
fn main() {
    println!("Guessing game");
    let sys_guess = rand::thread_rng().gen_range(1..=100);
    //println!("The random number is {guess}");
    loop {
        println!("Please enter your guess");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your Guess: {guess}");
        match guess.cmp(&sys_guess) {
            Ordering::Equal => {
                println!("You're right");
                break;
            },
            Ordering::Greater => println!("Too big"),
            Ordering::Less => println!("Too Small")
        }
    }
}
