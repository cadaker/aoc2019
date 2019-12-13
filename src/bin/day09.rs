use aoc2019::intcode;
use aoc2019::io::{slurp_stdin, parse_intcode_program};

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    {
        let mut out = vec![];
        intcode::run_program(program.clone(), &mut vec![1], &mut out).unwrap();
        assert_eq!(out.len(), 1);
        println!("{}", out[0]);
    }
    {
        let mut out = vec![];
        intcode::run_program(program.clone(), &mut vec![2], &mut out).unwrap();
        assert_eq!(out.len(), 1);
        println!("{}", out[0]);
    }
}
