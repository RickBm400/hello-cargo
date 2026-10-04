use std::io::stdin;  // standard library std
use rand::prelude::*;

fn main() {

    let secret_number = rand::rng().random_range(1..=100);

    println!("the secret number is {secret_number}");

    println!("guess the number");

    println!("please input your guest");

    let mut guess = String::new(); 

    stdin()
        .read_line(&mut guess) 
        .expect("Fail to read message"); // the & indicates that the string is being used as a reference 

    println!("Your guess is {guess}")

}
