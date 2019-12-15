use aoc2019::intcode;
use aoc2019::io::{slurp_stdin, parse_intcode_program};

fn main() {
    let data = parse_intcode_program(&slurp_stdin());
    let mut input: Vec<intcode::Mem> = vec![1];
    let mut output: Vec<intcode::Mem> = Vec::new();

    intcode::run_program_splitio(data.clone(), &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());

    input = vec![5];
    output = Vec::new();
    intcode::run_program_splitio(data, &mut input, &mut output).unwrap();
    println!("{}", output.iter().last().unwrap());
}
