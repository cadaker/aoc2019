use std::io::{self, Read};

pub fn parse_intcode_program(s: &String) -> Vec<i64> {
    s.trim()
        .split(",")
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn slurp_stdin() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");
    buf
}

