// use std::io; 
use std::net::TcpStream;

fn main() {
    if let Ok(_) = TcpStream::connect("scanme.nmap.org:80") {
        println!("Connection successful");
    } else {
        println!("Connection failed")
    }
    // println!("Simple Calculator in Rust");
    // println!("----------------------------");
    // println!("Enter the first number:");

    // let mut num1 = String::new();
    // io::stdin().read_line(&mut num1).expect("Failed to read line");

    // let num1: f64 = num1.trim().parse().expect("Please enter a valid number");

    // println!("Enter the second number:");

    // let mut num2 = String::new();
    // io::stdin().read_line(&mut num2).expect("Failed to read line");

    // let num2: f64 = num2.trim().parse().expect("Please enter a valid number");

    // println!("Enter the operator (+, -, *, /):");

    // let mut operator = String::new();
    // io::stdin().read_line(&mut operator).expect("Failed to read line");

    // let operator = operator.trim();

    // let result = match operator {
    //     "+" => num1 + num2,
    //     "-" => num1 - num2,
    //     "*" => num1 * num2,
    //     "/" => num1 / num2,
    //     _ => {
    //         println!("Invalid operator");
    //         return;
    //     }
    // };

    // println!("Result: {}", result);
}