use std::io;

fn fib(n: u128) -> u128 {
    let mut prev = 0;
    let mut curr = 1;
    let mut temp: u128;
    for _ in 0..n {
        temp = prev + curr;
        prev = curr;
        curr = temp;
    }
    return curr;
}

fn main() {
    println!("Please enter a positive number:");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let n: u128 = input
        .trim()
        .parse()
        .expect("Please enter a positive number");

    if n < 1 {
        panic!("Please enter a positive number");
    }

    println!("The {n}-th Fibonacci number is {}.", fib(n-1));

}
