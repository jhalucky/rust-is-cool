fn main() {
    println!("Hello Rust!");

    let number = {
        let x = 3;
        x+3
    };

    println!("Number: {}",number);
}