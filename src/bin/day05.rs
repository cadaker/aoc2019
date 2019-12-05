use aoc2019::intcode::run_program;
use std::io::{self, Read};

fn slurp_stdin() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");
    buf
}

fn main() {
    let data: Vec<i32> = slurp_stdin()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();
    let mut input = vec![1];
    let mut output = Vec::<i32>::new();

    run_program(data.clone(), &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());

    input = vec![5];
    output = Vec::new();
    run_program(data, &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());
}
