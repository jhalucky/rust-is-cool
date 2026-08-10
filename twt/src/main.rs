fn main() {
    // we use "mut" keyword to make any variable mutable because bydefault all variables declared in rust are immutable
    let mut x = 4;
    println!("x is: {}",x);

    x = 5;
    println!("x is: {}",x);

    let mut y = "raveesh";
    println!("y is: {}",y);

    y = "Raveesh";
    println!("y is: {}",y);
}


