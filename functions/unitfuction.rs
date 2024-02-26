use std::io;

fn sum(){
    println!("Enter the first number: ");
    let mut input = String::new();
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");

    let num1 : i32 = input.trim().parse().expect("Failed to parse input");

    println!("Enter the second number: ");

    let mut input2 = String::new();
    io::stdin()
  .read_line(&mut input2)
  .expect("Failed to read line");

    let num2 : i32 = input2.trim().parse().expect("Failed to parse input");
    let sum : i32 ;
    sum = num1 + num2;

    println!("The sum of {} and {} is {}.", num1, num2, sum);


}

fn main(){
    sum();
}