use aoc2019::io::{parse_intcode_program, slurp_stdin};
use aoc2019::intcode;
use std::cell::RefCell;
use std::collections::{VecDeque, HashMap, HashSet};

#[derive(Eq, Ord, PartialOrd, PartialEq, Copy, Clone)]
enum NodeState {
    Running,
    Done,
    Waiting(usize),
}

struct NetworkIO<'a> {
    addr: intcode::Mem,
    queues: &'a RefCell<HashMap<intcode::Mem, VecDeque<intcode::Mem>>>,
    state: NodeState,
    output_addr: Option<intcode::Mem>,
    output_x: Option<intcode::Mem>,
}

impl<'a> NetworkIO<'a> {
    fn new(addr: intcode::Mem, queues: &'a RefCell<HashMap<intcode::Mem, VecDeque<intcode::Mem>>>) -> Self {
        NetworkIO::<'a> { addr, queues, state: NodeState::Running, output_addr: None, output_x: None }
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
        if val == -1 {
            if let NodeState::Waiting(n) = self.state {
                self.state = NodeState::Waiting(n+1);
            } else {
                self.state = NodeState::Waiting(0);
            }
        } else {
            self.state = NodeState::Running;
        }
        //println!("input: {} got {}", self.addr, val);
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
            //println!("output: {} got {} {}", addr, x, y);
        }
    }
}

fn is_idle(queues: &HashMap<intcode::Mem, VecDeque<intcode::Mem>>, ios: &Vec<NetworkIO>) -> bool {
    for (addr, queue) in queues.iter() {
        if *addr != 255 && !queue.is_empty() {
            return false;
        }
    }
    for io in ios {
        if io.state == NodeState::Running {
            return false;
        } else if let NodeState::Waiting(n) = io.state {
            if n < 50 {
                return false;
            }
        }
    }
    true
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let queues = RefCell::new(HashMap::new());
    let mut programs = Vec::new();
    let mut ips = Vec::new();
    let mut rel_bases = Vec::new();
    let mut ios = Vec::new();
    for addr in 0..50 {
        programs.push(intcode::Memory::new(program.clone()));
        ips.push(0);
        rel_bases.push(0);
        ios.push(NetworkIO::new(addr, &queues));
        queues.borrow_mut().entry(addr).or_default().push_back(addr);
    }

    let mut first_nat_y  = None;
    let mut ys_injected = HashSet::new();
    let y_injected_twice;

    loop {
        //println!("Loop");
        {
            let mut queues_ref = queues.borrow_mut();
            if is_idle(&queues_ref, &ios) {
                let nat_queue = queues_ref.entry(255).or_default();
                assert!(!nat_queue.is_empty());
                let mut x = nat_queue.pop_front().unwrap();
                let mut y = nat_queue.pop_front().unwrap();
                if first_nat_y.is_none() {
                    first_nat_y = Some(y);
                }
                while !nat_queue.is_empty() {
                    x = nat_queue.pop_front().unwrap();
                    y = nat_queue.pop_front().unwrap();
                }
                //println!("Idle! Injecting {} {}", x, y);
                let queue_0 = queues_ref.entry(0).or_default();
                queue_0.push_back(x);
                queue_0.push_back(y);
                if ys_injected.contains(&y) {
                    y_injected_twice = y;
                    break;
                } else {
                    ys_injected.insert(y);
                }
            }
        }

        for i in 0..programs.len() {
            if ios[i].state != NodeState::Done {
                match intcode::step_program(&mut programs[i], ips[i], rel_bases[i], &mut ios[i]).unwrap() {
                    intcode::StepResult::Continue(ip, rel_base) => {
                        ips[i] = ip;
                        rel_bases[i] = rel_base;
                    },
                    intcode::StepResult::End => {
                        ios[i].state = NodeState::Done;
                    },
                }
            }
        }
    }

    println!("{}", first_nat_y.unwrap());
    println!("{}", y_injected_twice);
}
