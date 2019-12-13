use std::collections::HashMap;
use std::convert::TryFrom;
use aoc2019::intcode;
use std::fs::File;
use std::io::{self, Read, BufRead};
use std::cell::RefCell;

#[derive(Copy, Clone, Eq, PartialEq)]
enum GameElement {
    Empty,
    Wall,
    Block,
    HorizontalPaddle,
    Ball,
}

impl TryFrom<intcode::Mem> for GameElement {
    type Error = String;

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        use GameElement::*;
        match value {
            0 => Ok(Empty),
            1 => Ok(Wall),
            2 => Ok(Block),
            3 => Ok(HorizontalPaddle),
            4 => Ok(Ball),
            _ => Err(String::from("invalid game element"))
        }
    }
}

type Point = (i64,i64);
type GameBoard = HashMap<Point, GameElement>;

struct Parser<'a> {
    x: Option<i64>,
    y: Option<i64>,
    board: &'a RefCell<GameBoard>,
}

impl intcode::Output for Parser<'_> {
    fn next_output(&mut self, val: i64) {
        if self.x.is_none() {
            self.x = Some(val);
        } else if self.y.is_none() {
            self.y = Some(val);
        } else {
            let x = self.x.unwrap();
            let y = self.y.unwrap();
            self.x = None;
            self.y = None;
            if x == -1 {
                println!("Score: {}", val);
            } else {
                self.board.borrow_mut().insert((x, y), GameElement::try_from(val).unwrap());
            }
        }
    }
}

fn parse_intcode_program(s: String) -> Vec<intcode::Mem> {
    s.trim()
        .split(",")
        .map(|s| s.parse::<intcode::Mem>().unwrap())
        .collect()
}

fn read_game_board(program: Vec<intcode::Mem>) -> GameBoard {
    let board = RefCell::new(GameBoard::new());
    {
        let mut parser = Parser { x: None, y: None, board: &board };
        intcode::run_program(program, &mut vec![], &mut parser).unwrap();
    }
    let copy = board.borrow().clone();
    copy
}

struct GameInput<'a> {
    board: &'a RefCell<GameBoard>,
}

fn print_board(board: &GameBoard) {
    let mut y = 0i64;
    while board.get(&(0i64,y)).is_some() {
        let mut x = 0i64;
        loop {
            use GameElement::*;
            match board.get(&(x,y)) {
                Some(Empty) => print!(" "),
                Some(Wall) => print!("#"),
                Some(Block) => print!("."),
                Some(HorizontalPaddle) => print!("_"),
                Some(Ball) => print!("o"),
                None => break,
            }
            x += 1;
        }
        println!();
        y += 1;
    }
}

fn get_line() -> io::Result<String> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();

    handle.read_line(&mut buf)?;
    Ok(buf)
}

impl intcode::Input for GameInput<'_> {
    fn next_input(&mut self) -> Result<i64, String> {
        print_board(&self.board.borrow());

        loop {
            let input = get_line().or(Err(String::from("input failure")))?;

            match input.get(0..1) {
                Some("s") => return Ok(0),
                Some("a") => return Ok(-1),
                Some("d") => return Ok(1),
                _ => print!("invalid input")
            }
        }
    }
}

fn main() {
    let mut program_input = String::new();
    File::open("data/day13.in")
        .unwrap()
        .read_to_string(&mut program_input)
        .unwrap();
    let program = parse_intcode_program(program_input);
    let board = read_game_board(program.clone());

    println!("{}",
             board
                 .iter()
                 .filter(
                     |&(_p, val)|
                         *val == GameElement::Block)
                 .count());

    let mut prog = program;
    prog[0] = 2;
    let board = RefCell::new(GameBoard::new());
    let mut game_input = GameInput { board: &board };
    let mut parser = Parser { x: None, y: None, board: &board};
    intcode::run_program(prog, &mut game_input, &mut parser).unwrap();
}
