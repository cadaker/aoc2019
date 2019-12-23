use aoc2019::io::{parse_intcode_program, slurp_stdin};
use aoc2019::intcode;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap};

struct NetworkIO<'a> {
    addr: intcode::Mem,
    queues: &'a RefCell<HashMap<intcode::Mem, VecDeque<intcode::Mem>>>,
    output_addr: Option<intcode::Mem>,
    output_x: Option<intcode::Mem>,
}

impl<'a> NetworkIO<'a> {
    fn new(addr: intcode::Mem, queues: &'a RefCell<HashMap<intcode::Mem, VecDeque<intcode::Mem>>>) -> Self {
        NetworkIO::<'a> { addr, queues, output_addr: None, output_x: None }
    }
}

impl intcode::InputOutput for NetworkIO<'_> {
    fn next_input(&mut self) -> Result<i64, String> {
        let mut queues = self.queues.borrow_mut();
        let val = if let Some(queue) = queues.get_mut(&self.addr) {
            queue.pop_front().unwrap_or(-1)
        } else {
            -1
        };
        Ok(val)
    }

    fn next_output(&mut self, val: i64) {
        if self.output_addr.is_none() {
            self.output_addr = Some(val)
        } else if self.output_x.is_none() {
            self.output_x = Some(val)
        } else {
            let addr = self.output_addr.unwrap();
            let x = self.output_x.unwrap();
            let y = val;
            self.output_addr = None;
            self.output_x = None;

            let mut queues = self.queues.borrow_mut();
            let queue = queues.entry(addr).or_default();
            queue.push_back(x);
            queue.push_back(y);
        }
    }
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let queues = RefCell::new(HashMap::new());
    let mut programs = Vec::new();
    let mut ips = Vec::new();
    let mut rel_bases = Vec::new();
    let mut done = Vec::new();
    let mut ios = Vec::new();
    for addr in 0..50 {
        programs.push(intcode::Memory::new(program.clone()));
        ips.push(0);
        rel_bases.push(0);
        done.push(false);
        ios.push(NetworkIO::new(addr, &queues));
        queues.borrow_mut().entry(addr).or_default().push_back(addr);
    }

    loop {
        if queues.borrow().contains_key(&255) {
            let mut queues_ref = queues.borrow_mut();
            let queue = queues_ref.get_mut(&255).unwrap();
            let _x = queue.pop_front().unwrap();
            let y = queue.pop_front().unwrap();
            println!("{}", y);
            break;
        }
        for i in 0..programs.len() {
            if !done[i] {
                match intcode::step_program(&mut programs[i], ips[i], rel_bases[i], &mut ios[i]).unwrap() {
                    intcode::StepResult::Continue(ip, rel_base) => {
                        ips[i] = ip;
                        rel_bases[i] = rel_base;
                    },
                    intcode::StepResult::End => {
                        done[i] = true;
                    },
                }
            }
        }
    }
}
