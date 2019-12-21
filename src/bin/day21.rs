use aoc2019::io::{slurp_stdin, parse_intcode_program};
use aoc2019::intcode;

fn run_springcode(program: Vec<intcode::Mem>, springcode: &str) {
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
        for x in iter {
            print!("{}", x as u8 as char);
        }
    } else {
        println!("{}", ans);
    }
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    // (!A || !B || !C) && D
    let springcode_1 = "\
    NOT A J\n\
    NOT B T\n\
    OR T J\n\
    NOT C T\n\
    OR T J\n\
    AND D J\n\
    WALK\n";

    run_springcode(program.clone(), &springcode_1);

    // (!A || !B || !C) && D && !(!E && !H) =
    // !(A && B && C) && D && (E || H)
    let springcode_2 = "\
    NOT A J
    NOT J J
    AND B J
    AND C J
    NOT J J
    AND D J
    NOT E T
    NOT T T
    OR H T
    AND T J
    RUN\n";

    run_springcode(program, &springcode_2);
}
