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


fn main() {

    loop {

        println!("Enter first number: ");
        let mut a: String = String::new();
        io::stdin().read_line(&mut a).expect("Failde to read lline");
        let a: i64 = a.trim().parse().expect("Enter a number!");

        println!("Enter second number: ");
        let mut b: String = String::new();
        io::stdin().read_line(&mut b).expect("Failed to read line");
        let b: i64 = b.trim().parse().expect("Enter a number!");


        println!("A is addition");
        println!("S is subtraction");
        println!("M is multiply");
        println!("D is divide");
        println!("Q is exit");


        enum Operations {
            Add,
            Subtract,
            Multiply,
            Divide,
            Exit,
        }

        println!("Enter the operation: ");
        let mut input: String = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();


        let op = match input {
            "A" => Operations::Add,
            "S" => Operations::Subtract,
            "M" => Operations::Multiply,
            "D" => Operations::Divide,
            "Q" => Operations::Exit,
            _ => panic!("Unknown operation!"),
        };

        match op {
            Operations::Add => println!("{} + {} = {}", a, b, addition(a, b)),
            Operations::Subtract => println!("{} - {} = {}", a, b, subtraction(a, b)),
            Operations::Multiply => println!("{} * {} = {}", a, b, multiply(a, b)),
            Operations::Divide => println!("{} / {} = {}", a, b, divide(a, b)),
            Operations::Exit => break,
        };


    }
}
