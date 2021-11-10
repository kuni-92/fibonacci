use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let number = &args[1];
    let number: i32 = String::from(number).parse()
        .expect("Invalid args.");
    let result = fibonacci(number);
    println!("fbonacci {} is {}", number, result);
}

fn fibonacci(n : i32) -> i32 {
    if n < 2 {
        return n;
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2);
    }
}
