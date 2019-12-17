use aoc2019::io::{slurp_stdin, parse_intcode_program};
use aoc2019::intcode;
use std::fmt::{Formatter, Error, Write};

struct Map {
    map: Vec<char>,
    width: Option<usize>,
}

impl Map {
    fn new() -> Self {
        Map { map: vec![], width: None }
    }

    fn get_width(&self) -> usize {
        self.width.unwrap()
    }

    fn get_height(&self) -> usize {
        assert_ne!(self.get_width(), 0);
        assert_eq!(self.map.len() % self.get_width(), 0);
        self.map.len() / self.get_width()
    }

    fn get(&self, x: usize, y: usize) -> char {
        if x < self.get_width() && y < self.get_height() {
            *self.map.get(y * self.get_width() + x).unwrap()
        } else {
            '.'
        }
    }
}

impl std::fmt::Display for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        let width = self.get_width();
        for (i, c) in self.map.iter().enumerate() {
            f.write_char(*c).unwrap();
            if i % width == width - 1 {
                f.write_char('\n').unwrap();
            }
        }
        Ok(())
    }
}

impl intcode::Output for Map {
    fn next_output(&mut self, x: i64) {
        let c = x as u8 as char;
        if c == '\n' {
            match self.width {
                None => self.width = Some(self.map.len()),
                Some(w) => assert_eq!(self.map.len() % w, 0),
            }
        } else {
            self.map.push(c);
        }
    }
}

fn find_intersections(map: &Map) -> Vec<(usize, usize)> {
    fn is_scaffold(c: char) -> bool {
        c == '#' || c == '<' || c == '>' || c == '^' || c == 'v'
    }

    let mut ret = Vec::new();
    for y in 1..map.get_height() - 1 {
        for x in 1..map.get_width() - 1 {
            if is_scaffold(map.get(x, y)) &&
                is_scaffold(map.get(x - 1, y)) &&
                is_scaffold(map.get(x + 1, y)) &&
                is_scaffold(map.get(x, y - 1)) &&
                is_scaffold(map.get(x, y + 1))
            {
                ret.push((x, y));
            }
        }
    }
    ret
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let mut map = Map::new();
    intcode::run_program_splitio(program, &mut vec![], &mut map).unwrap();

    let intersections = find_intersections(&map);

    let alignments: usize = intersections.iter()
        .map(|(x, y)| *x * *y)
        .sum();

    println!("{}", alignments);
}
