use std::io;

fn main() {
    let x: i8 = 12; // u8 ranges: 0 - 255
    let y: i8 = -9; // i8 ranges: -128 - 127

    let z: i8 = x + y; // we changed one of type cuz we can't perform any arithmetic on different datatypes, we can't even add numbers with same bases but different sizes like i64 and i8, we can't perform any arithmetic
    println!("Sum: {}",z);
    // overflow();
    input_num();

}

fn overflow() {
    let a: u8 = 254;
    let b: u8 = 1;

    let c = a + b;
    // this will create a overflow condition - cuz c is of type u8, and adding a+b will result in 256, which is out of range!
    // so we need to adjust values so it would result under range.
    println!("Sum: {}",c)
}

fn input() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("expected to read a line!");

    println!("{}",input);
}

fn input_num() {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    let marks: i32 = input.trim().parse().expect("Enter the number");

    println!("I love rust {}",marks);
}