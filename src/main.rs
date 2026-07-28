use std::io;

fn addition(a: i64, b: i64) -> i64 {
    a + b
}

fn subtraction(a: i64, b: i64) -> i64 {
    a- b
}

fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn divide(a: i64, b: i64) -> i64 {
    a / b
}



fn main() {

    println!("Enter first number: ");
    let mut a: String = String::new();
    io::stdin().read_line(&mut a).expect("Failde to read lline");
    let a: i64 = a.trim().parse().expect("Enter a number!");

    println!("Enter second number: ");
    let mut b: String = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: i64 = b.trim().parse().expect("Enter a number!");


    let result_a: i64 = addition(a, b);
    let result_s: i64 = subtraction(a, b);
    let result_m: i64 = multiply(a, b);
    let result_d: i64 = divide(a, b);

    println!("{} + {} = {}", a, b, result_a);
    println!("{} - {} = {}", a, b, result_s);
    println!("{} * {} = {}", a, b, result_m);
    println!("{} / {} = {}", a, b, result_d);


}