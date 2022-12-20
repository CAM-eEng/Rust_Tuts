use std::io;

fn main() {
    println!("Please enter the index of the Fibonacci sequence you wish to calculate.");
    let mut index = String::new();
    io::stdin().read_line(&mut index).expect("Failed to read input");
    let index_val: u64 = index.trim().parse().expect("Please type a number!");
    println!("The fibonnaci number for {} is {}", index, fibo(index_val));
}

fn fibo(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibo(n-1) + fibo(n-2),
    }
}
