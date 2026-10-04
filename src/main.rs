use std::io::stdin;  // standard library std

fn main() {
    println!("guess the number");

    println!("please input your guest");

    let mut guess = String::new();

    stdin()
        .read_line(&mut guess)
        .expect("Fail to read message");

    println!("Your guess is {guess}")

}
