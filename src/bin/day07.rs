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

struct Queue {
    head: Vec<i32>,
    tail: Vec<i32>,
}

impl Queue {
    fn new() -> Self {
        Queue {head: Vec::new(), tail: Vec::new()}
    }
    fn push(&mut self, x: i32) {
        self.tail.push(x)
    }
    fn pop(&mut self) -> Option<i32> {
        if self.head.is_empty() {
            self.tail.reverse();
            std::mem::swap(&mut self.head, &mut self.tail);
        }
        self.head.pop()
    }
    fn is_empty(&self) -> bool {
        self.head.is_empty() && self.tail.is_empty()
    }
    #[allow(dead_code)]
    fn len(&self) -> usize {
        self.head.len() + self.tail.len()
    }
}

impl intcode::Input for Queue {
    fn next_input(&mut self) -> Result<i32, String> {
        self.pop().ok_or(String::from("no item to pop"))
    }
}

impl intcode::Output for Queue {
    fn next_output(&mut self, x: i32) {
        self.push(x)
    }
}

struct Context {
    memory: intcode::Memory,
    ip: usize,
    input_queue: Queue,
    done: bool,
}

fn all_done(contexts: &[Context]) -> bool {
    for c in contexts {
        if !c.done {
            return false;
        }
    }
    true
}

fn get_two(contexts: &mut Vec<Context>, ix0: usize, ix1: usize) -> (&mut Context, &mut Context) {
    assert_ne!(ix0, ix1);
    if ix0 < ix1 {
        let (head, tail) = contexts.split_at_mut(ix1);
        (&mut head[ix0], &mut tail[0])
    } else {
        let (head, tail) = contexts.split_at_mut(ix0);
        (&mut tail[0], &mut head[ix1])
    }
}

fn run_feedback(program: Vec<i32>, phases: Vec<i32>) -> Result<i32,String> {
    let mut contexts: Vec<Context> = Vec::new();
    for phase in phases {
        let mut context = Context {
            memory: intcode::Memory::new(program.clone()),
            ip: 0,
            input_queue: Queue::new(),
            done: false,
        };
        context.input_queue.push(phase);
        contexts.push(context);
    }

    contexts[0].input_queue.push(0);

    let mut ix = 0;
    while !all_done(&contexts) {
        let next_ix = (ix + 1) % contexts.len();

        let (cur_context, next_context) = get_two(&mut contexts, ix, next_ix);

        while !cur_context.done &&
            !(intcode::needs_input(&cur_context.memory, cur_context.ip)? &&
                cur_context.input_queue.is_empty()) {

            match intcode::step_program(
                &mut cur_context.memory,
                cur_context.ip,
                &mut cur_context.input_queue,
                &mut next_context.input_queue
            )? {
                intcode::StepResult::Continue(new_ip) => {
                    cur_context.ip = new_ip;
                },
                intcode::StepResult::End => {
                    cur_context.done = true;
                },
            }
        }
        ix = next_ix;
    }
    assert!(!contexts[0].input_queue.is_empty());
    contexts[0].input_queue.pop().ok_or(String::from("no input at end of feedback"))
}

fn best_feedback_signal(program: Vec<i32>) -> i32 {
    Permutations::new(vec![5, 6, 7, 8, 9])
        .map(|phases| -> i32 {
            run_feedback(program.clone(), phases).unwrap()
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

    println!("{}", best_signal(program.clone()));
    println!("{}", best_feedback_signal(program.clone()));
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
