use rand::Rng;
use std::cmp::Ordering;
use std::io;
//using librarires
fn main() {
    println!("Guess Game");

    //taking user input and modifying the string to enable comparison
    loop {
        println!("Enter a random number");
        let mut guess = String::new();
        let secret = rand::thread_rng().gen_range(1..=100);
        //generating random secret number
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed");
        //println!("Guessed Number: {guess}");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret) {
            //matching the guessed number to random number generated
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}
