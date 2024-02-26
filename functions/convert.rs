use std::io;

fn main() {
    println!("Please enter an integer:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let mut number: i32 = 0;
    let mut is_negative = false;
    let input = input.trim(); // Trim whitespace

    for (i, c) in input.chars().enumerate() {
        if i == 0 && c == '-' {
            is_negative = true;
            continue;
        }
        let digit = c.to_digit(10).expect("Invalid digit");
        number = number * 10 + digit as i32;
    }

    if is_negative {
        number *= -1;
    }

    println!("You entered: {}", number);
}
