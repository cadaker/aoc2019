type Mem = i32;
type Ptr = usize;

pub struct Memory {
    memory: Vec<Mem>
}

#[derive(Debug,PartialEq)]
enum Param {
    Pos(Ptr),
    Imm(Mem),
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
    fn next_output(&mut self, x: i32) {
        self.push(x)
    }
}

fn decode_param(m: &Memory, ptr: Ptr, opcode: Mem, index: u32) -> Result<Param, String> {
    let mut c = opcode / 100;
    for _ in 0..index {
        c /= 10
    }
    c %= 10;

    let &val = m.memory.get(ptr).ok_or(String::from("end of memory"))?;

    if c == 0 {
        Ok(Param::Pos(val as usize))
    } else if c == 1 {
        Ok(Param::Imm(val))
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
    match m.memory[ip] % 100 {
        1 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, m.memory[ip])?;
            Ok(Op::Add(p0, p1, p2))
        },
        2 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, m.memory[ip])?;
            Ok(Op::Mul(p0, p1, p2))
        },
        3 => {
            let p = decode_param(m, ip+1, m.memory[ip], 0)?;
            Ok(Op::In(p))
        },
        4 => {
            let p = decode_param(m, ip+1, m.memory[ip], 0)?;
            Ok(Op::Out(p))
        },
        5 => {
            let expr = decode_param(m, ip+1, m.memory[ip], 0)?;
            let dest = decode_param(m, ip+2, m.memory[ip], 1)?;
            Ok(Op::JumpIfTrue(expr, dest))
        },
        6 => {
            let expr = decode_param(m, ip+1, m.memory[ip], 0)?;
            let dest = decode_param(m, ip+2, m.memory[ip], 1)?;
            Ok(Op::JumpIfFalse(expr, dest))
        },
        7 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, m.memory[ip])?;
            Ok(Op::LessThan(p0, p1, p2))
        },
        8 => {
            let (p0, p1, p2) = decode_3_params(m, ip+1, m.memory[ip])?;
            Ok(Op::Equals(p0, p1, p2))
        },
        99 => {
            Ok(Op::End)
        },
        _ => Err(String::from("invalid opcode")),
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
}

////////////////////////////////////////////////////////////////

fn eval_param(p: &Param, m: &Memory) -> Mem {
    match *p {
        Param::Pos(ptr) => m.memory[ptr],
        Param::Imm(val) => val,
    }
}

fn write_param(val: Mem, p: &Param, m: &mut Memory) -> Result<(), String> {
    match *p {
        Param::Pos(ptr) => {
            m.memory[ptr] = val;
            Ok(())
        },
        Param::Imm(_) => Err(String::from("tried to write to immediate")),
    }
}

pub fn run_program(
    memdata: Vec<Mem>,
    input: &mut dyn Input,
    output: &mut dyn Output) -> Result<Vec<Mem>, String>
{
    let mut memory = Memory { memory: memdata };
    let mut ip: Ptr = 0;
    loop {
        let op = decode_instr(&memory, ip)?;
        ip = match op {
            Op::Add(lhs, rhs, dest) => {
                write_param(
                    eval_param(&lhs, &memory) + eval_param(&rhs, &memory),
                    &dest,
                    &mut memory)?;
                ip+4
            },
            Op::Mul(lhs, rhs, dest) => {
                write_param(
                    eval_param(&lhs, &memory) * eval_param(&rhs, &memory),
                    &dest,
                    &mut memory)?;
                ip+4
            },
            Op::In(p) => {
                write_param(input.next_input()?, &p, &mut memory)?;
                ip+2
            },
            Op::Out(p) => {
                output.next_output(eval_param(&p, &memory));
                ip+2
            },
            Op::JumpIfTrue(expr, dest) => {
                if eval_param(&expr, &memory) != 0 {
                    eval_param(&dest, &memory) as Ptr
                } else {
                    ip+3
                }
            },
            Op::JumpIfFalse(expr, dest) => {
                if eval_param(&expr, &memory) == 0 {
                    eval_param(&dest, &memory) as Ptr
                } else {
                    ip+3
                }
            },
            Op::LessThan(lhs, rhs, dest) => {
                write_param((eval_param(&lhs, &memory) < eval_param(&rhs, &memory)) as i32,
                            &dest,
                            &mut memory)?;
                ip+4
            },
            Op::Equals(lhs, rhs, dest) => {
                write_param((eval_param(&lhs, &memory) == eval_param(&rhs, &memory)) as i32,
                            &dest,
                            &mut memory)?;
                ip+4
            },
            Op::End => return Ok(memory.memory)
        };
    }
}

