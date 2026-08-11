use std::io;

fn main() {
    println!("Hello, Mr?");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    println!("Hello Mr. {}",input);
    // int();
    bool();

}

fn int() {
    println!("How much marks did you get in each subject");
    println!("In Enlish");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("failed to read line");

    let marks: i32 = input.trim().parse().expect("Enter a valid number");

    println!("In English: {}", marks);
}

fn bool() {
    println!("Is that true you started learning rust");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("failed to read line");

    let answer: bool = input.trim().parse().expect("Enter a boolean Value");

    println!("Yes, that's {}",answer);
}