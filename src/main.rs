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


        println!("+ is addition");
        println!("- is subtraction");
        println!("* is multiply");
        println!("/ is divide");
        println!("q is exit");

        println!("Enter the operation: ");
        let mut op: String = String::new();
        io::stdin().read_line(&mut op).expect("Failed to read line");
        let op = op.trim();



        match op {
            "+" => println!("{} + {} = {}", a, b, addition(a, b)),
            "-" => println!("{} - {} = {}", a, b, subtraction(a, b)),
            "*" => println!("{} * {} = {}", a, b, multiply(a, b)),
            "/" => println!("{} / {} = {}", a, b, divide(a, b)),
            _ => println!("Result: No operation"),
        }

        if op == "q" {
            break
        }
    }
}
