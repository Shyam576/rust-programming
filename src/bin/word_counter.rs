use std::io;

fn main() {
    println!(" Enter your text: ");

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let count = input.split_whitespace().count();

    println!("Word count : {}", count);
}
