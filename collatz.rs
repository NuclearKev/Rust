use std::io;

fn main() {

    let mut i = String::new();

    io::stdin().read_line(&mut i)
        .ok()
        .expect("failed to read line");

    let i: i32 = i.trim().parse()
        .ok()
        .expect("Please type a number!");

    println!("{} has {} Collatz steps", i, collatz(i));
}

fn collatz(n: i32) -> i32 {
    
    if n == 1 { return 0; }
    
    match n%2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(3*n + 1) }
    }
}
