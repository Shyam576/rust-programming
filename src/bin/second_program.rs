use std::io;

fn main(){
    println!("Simple Calculator");

    //Read first number
    let mut input1 = String::new();
    println!("Enter first number: ");
    io::stdin().read_line(&mut input1).expect("Failed to read line");
    let num1: f64 = input1.trim().parse().expect("Please type a number");

    //Read second number
    let mut input2 = String::new();
    println!("Enter second number");
    io::stdin().read_line(&mut input2).expect("Failed to read line");
    let num2:f64 = input2.trim().parse().expect("Please type anumber");

    //Read operation
    let mut op = String::new();
    println!("Enter operation (+,-, *, /):");
    io::stdin().read_line(&mut op).expect("Failed to read line");
    let op = op.trim();

    //calculate
    let result = match op {
        "+" => num1 + num2,
        "-" => num1 - num2,
        "*" => num1 * num2,
        "/" => num1 / num2,
        _ => {
            println!("Invalid operator");
            return;
        }
    };

    println!("Result:{}",result);
}
