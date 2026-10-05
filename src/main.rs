use rand::prelude::*;
use std::cmp::Ordering;
use std::io::stdin; // standard library std

fn main() {
    let secret_number = rand::rng().random_range(1..=100);

    println!("the secret number is {secret_number}");

    println!("guess the number");

    loop {
        println!("please input your guest");

        let mut guess = String::new();

        stdin().read_line(&mut guess).expect("Fail to read message"); // the & indicates that the string is being used as a reference 

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess is {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big"),
            Ordering::Equal => {
                println!("You Win");
                break;
            }
        }
    }
}
