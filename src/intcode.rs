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
    End,
}

#[derive(Debug,PartialEq)]
struct Instr {
    op: Op,
    new_ip: Ptr,
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

fn decode_instr(m: &Memory, ip: Ptr) -> Result<Instr, String> {
    match m.memory[ip] % 100 {
        1 => {
            let p0 = decode_param(m, ip+1, m.memory[ip], 0)?;
            let p1 = decode_param(m, ip+2, m.memory[ip], 1)?;
            let p2 = decode_param(m, ip+3, m.memory[ip], 2)?;
            Ok(Instr {op: Op::Add(p0, p1, p2), new_ip: ip+4})
        },
        2 => {
            let p0 = decode_param(m, ip+1, m.memory[ip], 0)?;
            let p1 = decode_param(m, ip+2, m.memory[ip], 1)?;
            let p2 = decode_param(m, ip+3, m.memory[ip], 2)?;
            Ok(Instr {op: Op::Mul(p0, p1, p2), new_ip: ip+4})
        },
        3 => {
            let p = decode_param(m, ip+1, m.memory[ip], 0)?;
            Ok(Instr {op: Op::In(p), new_ip: ip+2})
        }
        4 => {
            let p = decode_param(m, ip+1, m.memory[ip], 0)?;
            Ok(Instr {op: Op::Out(p), new_ip: ip+2})
        }
        99 => {
            Ok(Instr {op: Op::End, new_ip: ip+1})
        }
        _ => Err(String::from("invalid opcode"))
    }
}

#[test]
fn test_decode() {
    //use crate::intcode::{decode_instr, Memory};
    assert_eq!(decode_instr(&Memory{ memory: vec![1002,4,3,4,33] }, 0),
               Ok(Instr {
                   op: Op::Mul(Param::Pos(4), Param::Imm(3), Param::Pos(4)),
                   new_ip: 4
               }));
}
