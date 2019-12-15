use aoc2019::intcode;
use aoc2019::io::{parse_intcode_program, slurp_stdin};

fn simulate_input(mut mem: Vec<intcode::Mem>, noun: intcode::Mem, verb: intcode::Mem) -> intcode::Mem {
    mem[1] = noun;
    mem[2] = verb;

    let out_mem = intcode::run_program_splitio(mem, &mut vec![], &mut vec![]).unwrap();
    *out_mem.first().unwrap()
}

fn main() {
    let memory = parse_intcode_program(&slurp_stdin());

    println!("{}", simulate_input(memory.clone(), 12, 02));
    const TARGET: intcode::Mem = 19690720;
    for noun in 0.. {
        for verb in 0..=noun {
            if simulate_input(memory.clone(), noun, verb) == TARGET {
                println!("{}", 100*noun + verb);
                return
            }
        }
    }
}
