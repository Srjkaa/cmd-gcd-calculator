use std::io::{Write, stderr};
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if n > m {
            let t = m;
            m = n;
            n = t;
        }
        m %= n;
    }
    n
}

fn main() {
    let mut numbers: Vec<u64> = Vec::new();

    for arg in std::env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("Failed to parse argument")
        );
    }

    if numbers.len() == 0 {
        writeln!(stderr(), "Not found numbers provided via command line...").unwrap();
        std::process::exit(1);
    }

    let mut divider = numbers[0];
    for num in &numbers[1..] {
        divider = gcd(divider, *num);
    }

    println!("The greatest common divisor for {:?} is {}", numbers, divider);
}
