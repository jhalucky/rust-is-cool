


fn main() {
    // ques1
    // if is_prime(25) {
    //     println!("Prime Number");
    // } else {
    //     println!("Not prime number");
    // }

    // ques2
    let primes = sieve_of_eratosthenes(30);
    println!("{:?}",primes)
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

// using Sieve of Eratosthenes.

fn sieve_of_eratosthenes(n: usize) -> Vec<usize> {
    let mut is_prime = vec![true; n + 1];
    is_prime[0] = false;
    if n >= 1 {
        is_prime[1] = false;
    }

    let limit = (n as f64).sqrt() as usize;

    for i in 2..=limit {
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= n {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter(|&(_, &prime)| prime)
        .map(|(num, _)| num)
        .collect()
}



