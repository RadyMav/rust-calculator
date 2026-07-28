use std::io;
fn add(a: i64, b: i64) -> i64{
    a + b
}
fn multiply(a: i64, b: i64) -> i64 {
    a * b
}

fn minus(a: i64, b: i64) -> i64{
    a - b
}
fn decimal(a: i64, b: i64) -> i64 {
    a / b
}
fn main() {



    println!("Enter first number: ");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let a: i64 = a.trim().parse().expect("Please type a number!");

    println!("Enter second number: ");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");
    let b: i64 = b.trim().parse().expect("Please type a number!");



    let result: i64 = add(a, b);
    let results: i64 = minus(a, b);
    let resultm: i64 = multiply(a, b);
    let resultd: i64 = decimal(a, b);

    println!("{} + {} = {}", a, b,  result);
    println!("{} - {} = {}", a, b, results);
    println!("{} * {} = {}", a, b, resultm);
    println!("{} / {} = {}", a, b, resultd)

}
