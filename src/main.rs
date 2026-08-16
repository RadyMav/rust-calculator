use std::io;

fn addition(a: i64, b: i64) -> i64 {
    a + b
}

fn subtraction(a: i64, b: i64) -> i64 {
    a - b
}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn divide(a: i64, b: i64) -> f64 {
    a as f64 / b as f64
}

fn modulo(a: i64, b: i64) -> i64 {
    a % b
}

fn power(a: i64, b: i64) -> i64 {
    a.pow(b as u32)
}

fn read_number(prompt: &str) -> i64 {
    loop {
        println!("{} ", prompt);
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("failed to read line");

        match input.trim().parse::<i64>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a number!"),

        }
    }
}

enum Operations {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Power,
    Exit,
}

fn main() {

    loop {

        println!("A is addition");
        println!("S is subtraction");
        println!("M is multiply");
        println!("D is divide");
        println!("R is modulo");
        println!("P is power");
        println!("Q is exit");


        println!("Enter the operation: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        let input = input.to_uppercase();


        let op = match input.as_str() {
            "A" => Operations::Add,
            "S" => Operations::Subtract,
            "M" => Operations::Multiply,
            "D" => Operations::Divide,
            "R" => Operations::Modulo,
            "P" => Operations::Power,
            "Q" => Operations::Exit,
            _ => {
                println!("Unknown operation");
                continue;
            },
        };

        if let Operations::Exit = op {
            break;
        }

        let a:i64 = read_number("Enter first number");
        let b:i64 = read_number("Enter second number");


        match op {
            Operations::Add => println!("{} + {} = {}", a, b, addition(a, b)),
            Operations::Subtract => println!("{} - {} = {}", a, b, subtraction(a, b)),
            Operations::Multiply => println!("{} * {} = {}", a, b, multiply(a, b)),
            Operations::Divide if b == 0 => {
                println!("Division by zero is not available!");
                continue;
            }
            Operations::Divide => println!("{} / {} = {}", a, b, divide(a, b)),
            Operations:: Modulo if b == 0 => {
                println!("Division by zero is not available!");
                continue;
            }
            Operations::Modulo => println!("{} % {} = {}", a, b, modulo(a, b)),
            Operations::Power => println!("{} ^ {} = {}", a, b, power(a, b)),
            Operations::Exit => break,
        };

    }
}
