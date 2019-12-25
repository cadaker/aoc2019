use std::fs::File;
use std::io::{Read, BufRead};
use std::io;
use aoc2019::io::parse_intcode_program;
use aoc2019::intcode;
use std::collections::VecDeque;
use aoc2019::intcode::Input;

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

struct BasicInput {
    buf: VecDeque<char>,
}

impl intcode::Input for BasicInput {
    fn next_input(&mut self) -> Result<i64, String> {
        let c = self.buf.pop_front().ok_or(String::from("no input left"))?;
        Ok(c as u8 as i64)
    }
}

struct ConsoleInput {
    input: BasicInput,
}

impl intcode::Input for ConsoleInput {
    fn next_input(&mut self) -> Result<i64, String> {
        if self.input.buf.is_empty() {
            self.input = BasicInput { buf: to_queue(&get_line().unwrap()) };
        }
        let c = self.input.next_input()? as u8 as char;
        print!("{}", c);
        Ok(c as u8 as i64)
    }
}

struct Experimenter {
    input: BasicInput,
    current_room: String,
    rooms: Vec<String>,
}

impl Experimenter {
    fn new(script1: &str, script2: &str) -> Self {
        let mut queue = to_queue(script1);
        queue.append(&mut to_queue(script2));
        Experimenter {
            input: BasicInput { buf: queue },
            current_room: String::from(""),
            rooms: Vec::new()
        }
    }
}

impl intcode::InputOutput for Experimenter {
    fn next_input(&mut self) -> Result<i64, String> {
        self.input.next_input()
    }

    fn next_output(&mut self, x: i64) {
        let c = x as u8 as char;
        if c == '=' && self.current_room.is_empty() {
            self.current_room.push(c);
        } else if c == '\n' && !self.current_room.is_empty() {
            let mut tmp = String::from("");
            std::mem::swap(&mut tmp, &mut self.current_room);
            self.rooms.push(tmp);
        } else if !self.current_room.is_empty() {
            self.current_room.push(c);
        }
    }
}

const FULL_SCRIPT: &str = "\
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

const ITEMS: [&str; 8] = ["astrolabe", "hologram", "space law space brochure", "wreath", "hypercube", "cake", "coin", "food ration"];
const ROOM: &str = "== Security Checkpoint ==";

fn experiment_script(items: &[&str], experiment: usize, direction: &str) -> String {
    let mut ret = String::new();
    for (i, item) in items.iter().enumerate() {
        let mask = 1 << i;
        if experiment & mask == 0 {
            ret.push_str("drop ");
            ret.push_str(*item);
            ret.push_str("\n");
        }
    }
    ret.push_str(direction);
    ret.push_str("\n");
    ret
}

fn run_experiment(program: &Vec<intcode::Mem>) {
    for experiment in 0..(1 << ITEMS.len()) {
        let script2 = experiment_script(&ITEMS, experiment, "south");
        let mut experimenter = Experimenter::new(FULL_SCRIPT, &script2);
        let _ret = intcode::run_program(program.clone(), &mut experimenter);
        if experimenter.rooms.last().unwrap() != ROOM {
            println!("{}", experiment);
            println!("{}", script2);
            break;
        }
    }
}

fn play(program: &Vec<intcode::Mem>, script: &str) {
    let mut input = ConsoleInput { input: BasicInput { buf: to_queue(script) } };
    let mut output = ConsoleOutput {};
    intcode::run_program_splitio(program.clone(), &mut input, &mut output).unwrap();
}

fn main() {
    let mut program_input = String::new();
    File::open("data/day25.in")
        .unwrap()
        .read_to_string(&mut program_input)
        .unwrap();
    let program = parse_intcode_program(&program_input);

    //play(&program, "");
    run_experiment(&program);
}
