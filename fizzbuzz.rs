fn main() {

    let mut i = 1;

    while i < 101 {

        if i%3 == 0 && i%5 == 0 {
            println!("fizzbuzz");
        }
        else if i%3 == 0 {
            println!("fizz");
        }
        else if i%5 == 0 {
            println!("buzz");
        }
        else {
            println!("{}", i);
        }

        i += 1;
    }
}
