use std::fs::File;
use std::io::{Read, BufRead};
use std::io;
use aoc2019::io::parse_intcode_program;
use aoc2019::intcode;
use std::collections::VecDeque;

fn get_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    handle.read_line(&mut buf)?;
    Ok(buf)
}

struct ConsoleOutput {
}

impl intcode::Output for ConsoleOutput {
    fn next_output(&mut self, x: i64) {
        print!("{}", x as u8 as char);
    }
}

struct ConsoleInput {
    buf: std::collections::VecDeque<char>,
}

impl intcode::Input for ConsoleInput {
    fn next_input(&mut self) -> Result<i64, String> {
        if self.buf.is_empty() {
            let line = get_line().unwrap();
            for c in line.chars() {
                self.buf.push_back(c);
            }
        }
        Ok(self.buf.pop_front().unwrap() as u8 as i64)
    }
}

fn main() {
    let mut program_input = String::new();
    File::open("data/day25.in")
        .unwrap()
        .read_to_string(&mut program_input)
        .unwrap();
    let program = parse_intcode_program(&program_input);

    let mut input = ConsoleInput { buf: VecDeque::new() };
    let mut output = ConsoleOutput {};
    intcode::run_program_splitio(program, &mut input, &mut output).unwrap();
}
