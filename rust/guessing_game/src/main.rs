//use std::io;

fn main() {
    println!("Chess the number!");

    println!("Please input your guess.");

    let mut guess = String::new();

    std::io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let x = 5;

    println!("You guesse: {guess} and {}", x);
}
