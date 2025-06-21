use std::{env};
use std::str::FromStr;
use gcd_lib::gcd;

fn main() {
    let mut numbers = Vec::new();
    for arg in env::args().skip(1) {
        numbers.push(u64::from_str(&arg)
            .expect("Error parsing argument"))
    }

    if numbers.len() == 0 {
        eprintln!("Usage gcd NUMBER ...");
        std::process::exit(1);
    }

    let mut d = numbers[0];

    for m in &numbers[1..] {
        d = gcd(d, *m).unwrap();
    }

     println!("Greatest common divider for {:?} is {}", numbers, d);
}