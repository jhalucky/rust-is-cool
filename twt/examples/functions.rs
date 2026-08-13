use std::io;

fn main() {
    println!("Functions:");
    // addition();
    // concatenation();
    subtraction();
}

// concatenation isn't possible in rust
// fn concatenation() {
//     let a = "I";
//     let b = "am";

//     println!("{}",a+b);
// }

fn addition() {
    let mut a = String::new();
    let mut b = String::new();

    io::stdin().read_line(&mut a).expect("failed to read line");
    let a: i64 = a.trim().parse().expect("Enter a valid number: ");

    io::stdin().read_line(&mut b).expect("failed to read line");
    let b: i64 = b.trim().parse().expect("Enter a valid number: ");

    let sum = a + b;
    println!("Sum: {}",sum);
}

fn subtraction() {
    let mut x = String::new();
    let mut y = String::new();

    io::stdin().read_line(&mut x).expect("Failed to read");
    let x: i32 = x.trim().parse().expect("Enter a valid number");
    
    io::stdin().read_line(&mut y).expect("Failed to read");
    let y: i32 = y.trim().parse().expect("Enter a valid number");

    let sub = x - y;
    println!("Sub: {}",sub);

    
}