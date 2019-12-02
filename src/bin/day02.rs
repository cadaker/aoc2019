use std::io::{self, Read};

type Mem = usize;
type IP = usize;

enum State {
    Running(IP),
    Stopped(Mem),
}

struct Simulator {
    memory: Vec<Mem>,
}

const ADD: Mem = 1;
const MUL: Mem = 2;
const END: Mem = 99;

impl Simulator {
    fn new(mem: Vec<Mem>) -> Self {
        Simulator{memory: mem}
    }

    fn execute_one(&mut self, pc: IP) -> Result<State, (IP, Mem)> {
        fn get_ops(mem: &Vec<Mem>, pc: IP) -> (Mem, Mem, Mem) {
            (mem[pc+1], mem[pc+2], mem[pc+3])
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

fn simulate_input(mem: &Vec<Mem>, noun: Mem, verb: Mem) -> Mem {
    let mut sim = Simulator::new(mem.clone());
    sim.memory[1] = noun;
    sim.memory[2] = verb;

    let mut pc: IP = 0;
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

    let memory: Vec<Mem> = buf
        .trim()
        .split(",")
        .map(|s| { s.parse().expect("parse failure") })
        .collect();

    println!("{}", simulate_input(&memory, 12, 02));
    const TARGET: Mem = 19690720;
    for noun in 0usize.. {
        for verb in 0usize..=noun {
            if simulate_input(&memory, noun, verb) == TARGET {
                println!("{}", 100*noun + verb);
                return
            }
        }
    }
}
