


fn main() {
    // ques1
    // if is_prime(25) {
    //     println!("Prime Number");
    // } else {
    //     println!("Not prime number");
    // }

    // ques2
    // let primes = sieve_of_eratosthenes(30);
    // println!("{:?}",primes)

    // ques3
    // let nums = [10,5,20,14,9];
    // println!("{:?}",second_largest(&nums));

    // ques1();

    // cel_to_fahrenheit(54.0);

    // ques4
    let nums = [1,5,6,87,67,99,100];
    for i in nums {
        println!("{} is {}", i, even_or_odd(i))
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

fn second_largest(nums: &[i32]) -> Option<i32> {
    let mut largest = None;
    let mut second = None;

    for &num in nums {
        if largest.is_none() || num > largest.unwrap() {
            second = largest;
            largest = Some(num);
        } else if num < largest.unwrap() && (second.is_none() || num > second.unwrap()) {
            second = Some(num);
        }
    }
    second
}

fn ques1() {
    let mut count = 0;

    for i in 1..=3 {
        count+=i
    }
    println!("Count: {}",count);
}

fn cel_to_fahrenheit(c: f64) {
    let f: f64 = c * 9.0/5.0 + 32.0;
    println!("Fahrenheit: {}",f);
}

fn even_or_odd(n: i32) -> &'static str {
    if n % 2 == 0 {
        return "Even"
    } else {
        return "Odd"
    }
}