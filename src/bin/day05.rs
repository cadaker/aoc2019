use aoc2019::intcode::run_program;
use aoc2019::io::slurp_stdin;

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
