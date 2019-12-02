use std::io::{self, Read};

enum State {
    Running(usize),
    Stopped(usize),
}

struct Simulator {
    memory: Vec<usize>,
}

const ADD: usize = 1;
const MUL: usize = 2;
const END: usize = 99;

impl Simulator {
    fn new(mem: Vec<usize>) -> Self {
        Simulator{memory: mem}
    }

    fn execute_one(&mut self, pc: usize) -> Result<State, (usize, usize)> {
        fn get_ops(mem: &Vec<usize>, pc: usize) -> (usize, usize, usize) {
            (mem[pc+1 as usize], mem[pc+2 as usize], mem[pc+3 as usize])
        }
        match self.memory[pc] {
            ADD => {
                let (op1,op2,op3) = get_ops(&self.memory, pc);
                self.memory[op3] = self.memory[op1] + self.memory[op2];
                Ok(State::Running(pc+4))
            },
            MUL => {
                let (op1,op2,op3) = get_ops(&self.memory, pc);
                self.memory[op3] = self.memory[op1] * self.memory[op2];
                Ok(State::Running(pc+4))
            },
            END => {
                Ok(State::Stopped(self.memory[0]))
            }
            _ => Err((pc, self.memory[pc]))
        }
    }
}

fn simulate_input(mem: &Vec<usize>, noun: usize, verb: usize) -> usize {
    let mut sim = Simulator::new(mem.clone());
    sim.memory[1] = noun;
    sim.memory[2] = verb;

    let mut pc = 0usize;
    loop {
        match sim.execute_one(pc).expect("invalid opcode") {
            State::Running(next_pc) => {
                pc = next_pc
            },
            State::Stopped(mem0) => {
                break mem0
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");

    let memory: Vec<usize> = buf
        .trim()
        .split(",")
        .map(|s| { s.parse().expect("parse failure") })
        .collect();

    println!("{}", simulate_input(&memory, 12, 02));
    const TARGET: usize = 19690720;
    for noun in 0usize.. {
        for verb in 0usize..=noun {
            if simulate_input(&memory, noun, verb) == TARGET {
                println!("{}", 100*noun + verb);
                return
            }
        }
    }
}
