use std::collections::HashMap;
use std::convert::TryFrom;
use aoc2019::intcode;
use std::fs::File;
use std::io::Read;
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
            self.board.borrow_mut().insert((x,y), GameElement::try_from(val).unwrap());
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
}
