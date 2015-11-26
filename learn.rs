use std::*;

fn main(){
    /* Constant variables */
    let firstNumber  = 5;
    let secondNumber = 5.5;

    /* Normal variables */
    let mut thirdNumber   = 12.5;
    let mut fourthNumber  = 20;

    /* A tuple */
    let tuple = (4, 5.5, true, "bye");

    /* Conditionals */
    if firstNumber == 6 {
        println!("The first number is {}.", firstNumber);
    }
    else if secondNumber == 5.6 {
        println!("The second number is {}.", secondNumber);
    }
    else {
        println!("The third and fourth numbers are {} and {}.", thirdNumber, fourthNumber);
    }

    /* Pattern Matching (switch/case statement) */
    match firstNumber {
        4 => { println!("Aye, the number is {}.", firstNumber); }
        5 => { println!("The number is right!"); }
        _ => { println!("Nothing is right!!!!"); }
    }

    /* Advanced match statement */
    match fourthNumber {
        20...29     => { println!("The number is in the twenties!"); }
        13|14|15|16 => { println!("The number is at a horrible age!"); }
        _           => { println!("Who knows where the number is..."); }
    }

    /* Using a tuple in a match statment */
    let newTup = (thirdNumber, true);
    match newTup {
        (11.0, true)  => { println!("It's true somewhere below 12..."); }
        (11.0, false) => { println!("It's not below 12!"); }
        (_, true)     => { println!("It's definitely a number!"); }
        (_, false)    => { println!("It's not a number!"); }
        // _             => { println!("Oh no!"); }
    }

    /* If and match combo! */
    let anotherTup = (firstNumber, fourthNumber);
    match anotherTup {
        (firstNumber, fourthNumber) if firstNumber > fourthNumber => { println!("The first number is bigger."); }
        (firstNumber, fourthNumber) if firstNumber < fourthNumber => { println!("The second number is bigger."); }
        _                                                         => { println!("Okay."); }
    }

    /* Test! */
    let yat = (100, false);
    match yat {
        (20...26, true) => { println!("True AND in range!"); }
        (_, true)       => { println!("Not in range, but still true"); }
        (40...49, _)    => { println!("In a different range and unsure if true or not."); }
        _               => { println!("I DON'T KNOW!"); }
    }


    /* Looping! */

    /* While */
    let mut i = 0;
    while i < 10 {
        println!("Hello Yinz!");
        i += 1; //rust doesn't support ++ or -- like in C/C++
    }

    /* Loop */
    /* This command is like while(1) or while true, use break to get out */
    loop {
        break;
    }

    /* For */
    // i = 0;
    // for i in range(0, 10) { //the "range" function doesn't exist?
    //     println!("Goodbye!");
    // }

    /* Expressions */
    let mut x = 5;
    let foo = if x == 5 {
        "five"                  // note that there isn't a semicolon
    }
    else if x == 6 {
        "six"
    }
    else {
        "neither"
    };                          // note the semicolon is here

    println!("{}", foo);

    /* Expressions in a match statement */
    let foo2 = match foo {
        "five" => { "Foo is five" }
        "six"  => { "Foo is six." }
        _      => { "Foo is undefined" }
    };

    println!("{}", foo2);

    prime_Sum(2, 3, 4);

    println!("{}", square(2));
}

/* Functions */

/* A void function */
fn prime_Sum(x: i8, y: i8, m: i8) {
    match (x+y)%m {
        0 => { println!("Multiple"); }
        _ => { println!("Relatively prime"); }
    }
}

/* A function that returns an int */
fn square(x: i8) -> i8 {
    x*x                         // note there is no semicolon
}
