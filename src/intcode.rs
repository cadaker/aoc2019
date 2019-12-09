pub type Mem = i64;
pub type Ptr = usize;

pub struct Memory {
    memory: Vec<Mem>
}

impl Memory {
    pub fn new(memory: Vec<Mem>) -> Self {
        Memory {memory}
    }

    fn read(&self, ptr: Ptr) -> Mem {
        self.memory.get(ptr).cloned().unwrap_or(0)
    }

    fn write(&mut self, ptr: Ptr, val: Mem) {
        if ptr >= self.memory.len() {
            self.memory.resize(ptr+1, 0);
        }
        self.memory[ptr] = val;
    }

    fn read_param(&self, param: &Param, rel_base: Mem) -> Mem {
        match *param {
            Param::Pos(ptr) => self.read(ptr),
            Param::Imm(val) => val,
            Param::Rel(adj) => self.read((rel_base+adj) as Ptr),
        }
    }

    fn write_param(&mut self, param: &Param, value: Mem, rel_base: Mem) -> Result<(), String> {
        match *param {
            Param::Pos(ptr) => Ok(self.write(ptr, value)),
            Param::Imm(_) => Err(String::from("writing to immediate")),
            Param::Rel(adj) => Ok(self.write((rel_base+adj) as Ptr, value)),
        }
    }
}

#[derive(Debug,PartialEq)]
enum Param {
    Pos(Ptr),
    Imm(Mem),
    Rel(Mem),
}

#[derive(Debug,PartialEq)]
enum Op {
    Add(Param, Param, Param),
    Mul(Param, Param, Param),
    In(Param),
    Out(Param),
    JumpIfTrue(Param, Param),
    JumpIfFalse(Param, Param),
    LessThan(Param, Param, Param),
    Equals(Param, Param, Param),
    AdjustRelBase(Param),
    End,
}

pub trait Input {
    fn next_input(&mut self) -> Result<Mem, String>;
}

pub trait Output {
    fn next_output(&mut self, x: Mem);
}

impl Input for Vec<Mem> {
    fn next_input(&mut self) -> Result<Mem, String> {
        self.pop().ok_or(String::from("not enough inputs in vector"))
    }
}

impl Output for Vec<Mem> {
    fn next_output(&mut self, x: Mem) {
        self.push(x)
    }
}

fn get_flag(opcode: Mem, index: u32) -> i32 {
    let mut flags = opcode/100;
    for _ in 0..index {
        flags /= 10
    }
    (flags % 10) as i32
}

fn decode_param(m: &Memory, ptr: Ptr, opcode: Mem, index: u32) -> Result<Param, String> {
    let val = m.read(ptr);

    let flag = get_flag(opcode, index);
    if flag == 0 {
        Ok(Param::Pos(val as Ptr))
    } else if flag == 1 {
        Ok(Param::Imm(val))
    } else if flag == 2 {
        Ok(Param::Rel(val))
    } else {
        Err(String::from("invalid parameter mode"))
    }
}

fn decode_3_params(m: &Memory, ptr: Ptr, opcode: Mem) -> Result<(Param,Param,Param), String> {
    let p0 = decode_param(m, ptr, opcode, 0)?;
    let p1 = decode_param(m, ptr+1, opcode, 1)?;
    let p2 = decode_param(m, ptr+2, opcode, 2)?;
    Ok((p0,p1,p2))
}

fn decode_instr(m: &Memory, ip: Ptr) -> Result<Op, String> {
    let opcode = m.read(ip);
    match opcode % 100 {
        1 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, opcode)?;
            Ok(Op::Add(p0, p1, p2))
        },
        2 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, opcode)?;
            Ok(Op::Mul(p0, p1, p2))
        },
        3 => {
            let p = decode_param(m, ip+1, opcode, 0)?;
            Ok(Op::In(p))
        },
        4 => {
            let p = decode_param(m, ip+1, opcode, 0)?;
            Ok(Op::Out(p))
        },
        5 => {
            let expr = decode_param(m, ip+1, opcode, 0)?;
            let dest = decode_param(m, ip+2, opcode, 1)?;
            Ok(Op::JumpIfTrue(expr, dest))
        },
        6 => {
            let expr = decode_param(m, ip+1, opcode, 0)?;
            let dest = decode_param(m, ip+2, opcode, 1)?;
            Ok(Op::JumpIfFalse(expr, dest))
        },
        7 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, opcode)?;
            Ok(Op::LessThan(p0, p1, p2))
        },
        8 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, opcode)?;
            Ok(Op::Equals(p0, p1, p2))
        },
        9 => {
            let amount = decode_param(m, ip+1, opcode, 0)?;
            Ok(Op::AdjustRelBase(amount))
        },
        99 => {
            Ok(Op::End)
        },
        _ => Err(String::from("invalid opcode")),
    }
}

////////////////////////////////////////////////////////////////

pub enum StepResult {
    Continue(Ptr, Mem), // ip, rel_base
    End,
}

pub fn step_program(
    mem: &mut Memory,
    ip: Ptr,
    rel_base: Mem,
    input: &mut dyn Input,
    output: &mut dyn Output) -> Result<StepResult, String>
{
    let op = decode_instr(&mem, ip)?;
    let (new_ip, new_rel_base) = match op {
        Op::Add(lhs, rhs, dest) => {
            mem.write_param(
                &dest,
                mem.read_param(&lhs, rel_base) + mem.read_param(&rhs, rel_base),
                rel_base)?;
            (ip+4, rel_base)
        },
        Op::Mul(lhs, rhs, dest) => {
            mem.write_param(
                &dest,
                mem.read_param(&lhs, rel_base) * mem.read_param(&rhs, rel_base),
                rel_base)?;
            (ip+4, rel_base)
        },
        Op::In(p) => {
            mem.write_param(&p, input.next_input()?, rel_base)?;
            (ip+2, rel_base)
        },
        Op::Out(p) => {
            output.next_output(mem.read_param(&p, rel_base));
            (ip+2, rel_base)
        },
        Op::JumpIfTrue(expr, dest) => {
            if mem.read_param(&expr, rel_base) != 0 {
                (mem.read_param(&dest, rel_base) as Ptr, rel_base)
            } else {
                (ip+3, rel_base)
            }
        },
        Op::JumpIfFalse(expr, dest) => {
            if mem.read_param(&expr, rel_base) == 0 {
                (mem.read_param(&dest, rel_base) as Ptr, rel_base)
            } else {
                (ip+3, rel_base)
            }
        },
        Op::LessThan(lhs, rhs, dest) => {
            mem.write_param(
                &dest,
                (mem.read_param(&lhs, rel_base) < mem.read_param(&rhs, rel_base)) as Mem,
                rel_base)?;
            (ip+4, rel_base)
        },
        Op::Equals(lhs, rhs, dest) => {
            mem.write_param(
                &dest,
                (mem.read_param(&lhs, rel_base) == mem.read_param(&rhs, rel_base)) as Mem,
                rel_base)?;
            (ip+4, rel_base)
        },
        Op::AdjustRelBase(adjustment) => {
            (ip+2, rel_base+mem.read_param(&adjustment, rel_base))
        },
        Op::End => return Ok(StepResult::End)
    };
    return Ok(StepResult::Continue(new_ip, new_rel_base))
}

pub fn run_program(
    memdata: Vec<Mem>,
    input: &mut dyn Input,
    output: &mut dyn Output) -> Result<Vec<Mem>, String>
{
    let mut mem = Memory { memory: memdata };
    let mut ip: Ptr = 0;
    let mut rel_base: Mem = 0;
    loop {
        let (new_ip,new_rel_base) = match step_program(&mut mem, ip, rel_base, input, output)? {
            StepResult::Continue(new_ip, new_rel_base) => (new_ip, new_rel_base),
            StepResult::End => return Ok(mem.memory)
        };
        ip = new_ip;
        rel_base = new_rel_base;
    }
}

pub fn needs_input(mem: &Memory, ip: Ptr) -> Result<bool,String> {
    match decode_instr(mem, ip)? {
        Op::In(_) => Ok(true),
        _ => Ok(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode() {
        assert_eq!(decode_instr(&Memory { memory: vec![1002, 4, 3, 4, 33] }, 0),
                   Ok(Op::Mul(Param::Pos(4), Param::Imm(3), Param::Pos(4))));
    }

    #[test]
    fn test_programs() {
        let mem = vec![1, 5, 6, 7,   // ADD [5] [6] -> [7]
                                99, 12, 18, 66];
        let expected = vec![1, 5, 6, 7, 99, 12, 18, 30];
        let res = run_program(mem, &mut vec![], &mut vec![]).unwrap();
        assert_eq!(res, expected);
    }

    #[test]
    fn test_memory() {
        let mem = Memory { memory: vec![1, 2, 3, 4] };
        assert_eq!(mem.read(2), 3);
        assert_eq!(mem.read(119), 0);
    }
}
