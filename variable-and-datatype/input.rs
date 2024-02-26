use std::io;

fn main() {
    println!("enter a world");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("no input string");

    println!("{}", guess);
}