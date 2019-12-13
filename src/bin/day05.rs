use aoc2019::intcode::{run_program,Mem};
use aoc2019::io::{slurp_stdin, parse_intcode_program};

fn main() {
    let data = parse_intcode_program(&slurp_stdin());
    let mut input: Vec<Mem> = vec![1];
    let mut output: Vec<Mem> = Vec::new();

    run_program(data.clone(), &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());

    input = vec![5];
    output = Vec::new();
    run_program(data, &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());
}
