


fn main() {
    if is_prime(25) {
        println!("Prime Number");
    } else {
        println!("Not prime number");
    }
}

fn is_prime(x: u32) -> bool{
    // for i in 2..x{
    //     if x % i == 0 {
    //         println!("Not prime!");
    //     } else {
    //         println!("Prime Number");
    //     }
    // }

    if x <= 1 {
        false
    } else if x == 2 {
        true
    } else if x % 2 == 0 {
        false
    } else {
        let limit = (x as f64).sqrt() as u32;
        let mut prime = true;

        for i in 3..=limit {
            if x % i == 0 {
                prime = false;
                break;
            }
        }
        prime
    }
}