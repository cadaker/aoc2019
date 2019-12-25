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

fn to_queue(s: &str) -> VecDeque<char> {
    let mut ret = VecDeque::new();
    for c in s.chars() {
        ret.push_back(c);
    }
    ret
}

struct ConsoleOutput {
}

impl intcode::Output for ConsoleOutput {
    fn next_output(&mut self, x: i64) {
        print!("{}", x as u8 as char);
    }
}

struct ConsoleInput {
    buf: VecDeque<char>,
}

impl intcode::Input for ConsoleInput {
    fn next_input(&mut self) -> Result<i64, String> {
        if self.buf.is_empty() {
            self.buf = to_queue(&get_line().unwrap());
        }
        let c = self.buf.pop_front().unwrap();
        print!("{}", c);
        Ok(c as u8 as i64)
    }
}

const SCRIPT: &str = "\
south\n\
take astrolabe\n\
west\n\
take hologram\n\
south\n\
take space law space brochure\n\
west\n\
take wreath\n\
west\n\
take hypercube\n\
east\n\
east\n\
north\n\
east\n\
south\n\
take cake\n\
west\n\
north\n\
take coin\n\
south\n\
east\n\
east\n\
south\n\
east\n\
take food ration\n\
south\n";

fn main() {
    let mut program_input = String::new();
    File::open("data/day25.in")
        .unwrap()
        .read_to_string(&mut program_input)
        .unwrap();
    let program = parse_intcode_program(&program_input);

    let mut input = ConsoleInput { buf: to_queue(SCRIPT) };
    let mut output = ConsoleOutput {};
    intcode::run_program_splitio(program, &mut input, &mut output).unwrap();
}
