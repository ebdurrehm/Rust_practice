use rand::{thread_rng, Rng};
use std::cmp::Ordering; //comparing values
use std::io;

fn main() {
    println!("Guess the number!");
    // array with explicit type annotation
    let arr: [i32; 3] = [1,2,3];
    
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

fn function_returns_expression (a:i32, b:i32) -> i32 {
    //  block of the code with the conditions called "arms"
    if true {
        a + b
    } else {
        0
    }

}