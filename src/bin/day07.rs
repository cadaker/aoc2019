use aoc2019::io::slurp_stdin;
use aoc2019::intcode;
use aoc2019::permutation::Permutations;

fn run_phase(program: Vec<i32>, phases: Vec<i32>) -> Result<i32, String> {
    let mut last_output = 0;
    for phase in phases {
        let mut input = Vec::<i32>::new();
        let mut output = Vec::<i32>::new();
        let mut machine = intcode::Memory::new(program.clone());

        input.push(last_output);
        input.push(phase);
        let mut ip = 0;
        while output.is_empty() {
            ip = match intcode::step_program(&mut machine, ip, &mut input, &mut output)? {
                intcode::StepResult::Continue(ptr) => ptr,
                intcode::StepResult::End => return Err(String::from("premature end of program")),
            }
        }
        last_output = output[0];
    }
    Ok(last_output)
}

fn best_signal(program: Vec<i32>) -> i32 {
    Permutations::new(vec![0, 1, 2, 3, 4])
        .map(|phases| -> i32 {
            run_phase(program.clone(), phases).unwrap()
        })
        .max()
        .unwrap()
}

fn main() {
    let program: Vec<i32> = slurp_stdin()
        .trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect();

    println!("{}", best_signal(program));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_program_examples() {
        {
            let prog = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
            assert_eq!(run_phase(prog, vec![4,3,2,1,0]).unwrap(), 43210);
        }
        {
            let prog = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
            assert_eq!(run_phase(prog, vec![0,1,2,3,4]).unwrap(), 54321);
        }
        {
            let prog = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
            assert_eq!(run_phase(prog, vec![1,0,4,3,2]).unwrap(), 65210);
        }
    }

    #[test]
    fn test_maximization_examples() {
        {
            let prog = vec![3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0];
            assert_eq!(best_signal(prog), 43210);
        }
        {
            let prog = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0];
            assert_eq!(best_signal(prog), 54321);
        }
        {
            let prog = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
            assert_eq!(best_signal(prog), 65210);
        }
    }
}
