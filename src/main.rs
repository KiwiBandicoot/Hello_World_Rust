use std::io::{self, Write};

//figure out how to use rust and have a play with
//the language and different functions! 

fn print_name_given() {
    print!("Please enter your name: ");
    io::stdout().flush().unwrap();
    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();
    greet(name);
    boots();
    println!("Bonjour, {}!", name);
}

fn bracket(a: i32, b: i32) -> i32 {
    a * b + a - b
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn boots(){
    let x: i32 = 10;
    let y: i32 = -5;
    
    println!("x: {}, y: {}, z: {}", x, y, x+y);
}

fn main() {
    print_name_given();
    let result = bracket(6, 2);
    println!("The sum is: {}", result);
    boots();
}