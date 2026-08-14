fn main() {
    println!("Hello Rust!");

    // let number = {
    //     let x = 3;
    //     x+3
    // };

    // println!("Number: {}",number);

    let result = add_numbers(95, 67);
    println!("{}",result);

    // result = 89 + 67;
    // println!("{}",result);

}

fn add_numbers(x: i32, y: i32) -> i32 {
    let result = x + y;
    if result > 200 {
        return result - 200;
    }
    result
}