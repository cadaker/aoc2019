use std::collections::HashMap;
use std::convert::TryFrom;
use aoc2019::intcode;
use aoc2019::io::slurp_stdin;

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

struct Parser {
    x: Option<i64>,
    y: Option<i64>,
    board: GameBoard,
}

impl intcode::Output for Parser {
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
            self.board.insert((x,y), GameElement::try_from(val).unwrap());
        }
    }
}

fn read_game_board() -> GameBoard {
    let program: Vec<intcode::Mem> = slurp_stdin()
        .trim()
        .split(",")
        .map(|s| s.parse::<intcode::Mem>().unwrap())
        .collect();
    let mut parser = Parser {x: None, y: None, board: GameBoard::new()};
    intcode::run_program(program, &mut vec![], &mut parser).unwrap();
    parser.board
}

fn main() {
    let board = read_game_board();

    println!("{}",
             board
                 .iter()
                 .filter(
                     |&(_p, val)|
                         *val == GameElement::Block)
                 .count());
}
