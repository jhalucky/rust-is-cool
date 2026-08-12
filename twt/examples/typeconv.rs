fn main() {
    let x: i8 = 12; // u8 ranges: 0 - 255
    let y: i8 = -9; // i8 ranges: -128 - 127

    let z: i8 = x + y; // we changed one of type cuz we can't perform any arithmetic on different datatypes, we can't even add numbers with same bases but different sizes like i64 and i8, we can't perform any arithmetic
    println!("Sum: {}",z);
}

