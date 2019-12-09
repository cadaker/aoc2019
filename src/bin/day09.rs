use aoc2019::intcode;
use aoc2019::io::slurp_stdin;

fn main() {
    let program: Vec<intcode::Mem> = slurp_stdin()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

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
