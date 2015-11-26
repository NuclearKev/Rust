use std::io;

fn main() {

    let mut i = String::new();

    io::stdin().read_line(&mut i)
        .ok()
        .expect("failed to read line");

    let i: i32 = i.trim().parse()
        .ok()
        .expect("Please type a number!");

    println!("{} is the smallest number that has {} steps", increment_numbers(i), i);
}

fn increment_numbers(x: i32) -> i32 {

    let mut i = 1;
    loop {
        if collatz(i) == x {
            return i;
        }

        i += 1;
    }
}

fn collatz(n: i32) -> i32 {
    
    if n == 1 { return 0; }
    
    match n%2 {
        0 => { 1 + collatz(n/2) }
        _ => { 1 + collatz(3*n + 1) }
    }
}

