use rand::{thread_rng, Rng};
use std::cmp::Ordering; //comparing values
use std::io;

fn main() {
    println!("Guess the number!");
    loop {
        println!("Please input your guess.");
        let secret_number = thread_rng().gen_range(1..100);
        println!("secret number: {secret_number}");
        let mut num = String::new();
        let input = io::stdin();
        input.read_line(&mut num).expect("Failed to read");

        let num: u32 = match num.trim().parse() {
            Ok(result) => result,
            Err(_) => continue,
        };
        match num.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
            Ordering::Greater => println!("Too big!"),
        }
    }
}
