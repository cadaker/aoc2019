use aoc2019::io::{slurp_stdin, parse_intcode_program};
use aoc2019::intcode;

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    // (!A || !B || !C) && D
    let springcode ="\
    NOT A J\n\
    NOT B T\n\
    OR T J\n\
    NOT C T\n\
    OR T J\n\
    AND D J\n\
    WALK\n";

    let mut input = Vec::new();
    for c in springcode.chars() {
        input.push(c as u8 as intcode::Mem);
    }
    input.reverse();

    let mut output = Vec::<intcode::Mem>::new();
    intcode::run_program_splitio(program, &mut input, &mut output).unwrap();
    let mut iter = output.into_iter().peekable();

    for _ in 0..4 {
        while iter.next().unwrap() as u8 as char != '\n' {}
    }
    let ans = *iter.peek().unwrap();
    if 0 <= ans && ans < 256 {
        println!("{}", ans);
        for x in iter {
            print!("{}", x as u8 as char);
        }
    } else {
        println!("{}", ans);
    }
}
