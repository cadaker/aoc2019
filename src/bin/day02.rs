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

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");

    let mut memory: Vec<usize> = buf
        .trim()
        .split(",")
        .map(|s| { s.parse().expect("parse failure") })
        .collect();

    memory[1] = 12;
    memory[2] = 02;
    let mut sim = Simulator::new(memory);
    let mut pc = 0usize;
    loop {
        match sim.execute_one(pc).expect("invalid opcode") {
            State::Running(next_pc) => {
                pc = next_pc
            },
            State::Stopped(mem0) => {
                println!("{}", mem0);
                break
            }
        }
    }
}
