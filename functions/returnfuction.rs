use std::io;

fn avarage(number: &[u64]) -> u64{
    let mut result : u64 = 0;
    for i in number{
        result += i;
    }
    println!("The sum of the numbers is {}",result);
    let avarage = result/number.len() as u64;
    avarage
    
    
}



fn main(){
    let mut input = String::new();
    println!("Enter the number of elements: ");
    io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
    let input : u64 = input.trim().parse().expect("Failed to parse input");
    let mut numbers = Vec::new();
    for i in 0..input{
        let mut input = String::new();
        println!("Enter the {} number: ",i+1);
        io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
        let input : u64 = input.trim().parse().expect("Failed to parse input");
        numbers.push(input);
    }
    println!("{:?}", numbers);
    let result = avarage(&numbers);

    println!("The avarage of the numbers is {}",result);
    
    let sum : u64 = numbers.iter().sum();
    println!("The sum of the numbers is {}",sum);

}