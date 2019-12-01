use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let w: u64 = handle
        .lines()
        .map(|s| { s.unwrap().parse::<u64>() })
        .map( |n| { (n.unwrap() / 3) - 2 })
        .sum();
    println!("{}", w);
}
