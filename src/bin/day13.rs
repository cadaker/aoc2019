use std::collections::HashMap;
use std::convert::TryFrom;
use aoc2019::intcode;
use aoc2019::io::parse_intcode_program;
use std::fs::File;
use std::io::{self, Read, BufRead};
use aoc2019::intcode::Output;

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
    score: Option<i64>,
}

impl Parser {
    fn new() -> Self {
        Parser { x: None, y: None, board: GameBoard::new(), score: None }
    }
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
            if x == -1 && y == 0 {
                self.score = Some(val);
            } else {
                self.board.insert((x, y), GameElement::try_from(val).unwrap());
            }
        }
    }
}

fn read_game_board(program: Vec<intcode::Mem>) -> GameBoard {
    let mut parser = Parser::new();
    intcode::run_program_splitio(program, &mut vec![], &mut parser).unwrap();
    parser.board
}

struct GameInput {
    parser: Parser,
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

impl intcode::InputOutput for GameInput {
    fn next_input(&mut self) -> Result<i64, String> {
        print_board(&self.parser.board);

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

    fn next_output(&mut self, x: i64) {
        self.parser.next_output(x)
    }
}

struct AiInput {
    parser: Parser,
}

fn find_elem(board: &GameBoard, sought_elem: GameElement) -> Vec<Point> {
    board
        .iter()
        .filter(|&(_p, elem)| *elem == sought_elem)
        .map(|(p,_elem)| *p)
        .collect()
}

fn find_single(board: &GameBoard, elem: GameElement) -> Option<Point> {
    let elems = find_elem(board, elem);
    elems.first().cloned()
}

impl intcode::InputOutput for AiInput {
    fn next_input(&mut self) -> Result<i64, String> {
        print_board(&self.parser.board);
        std::thread::sleep(std::time::Duration::from_millis(1));

        let ball = find_single(&self.parser.board, GameElement::Ball)
            .ok_or(String::from("no ball"))?;
        let paddle = find_single(&self.parser.board, GameElement::HorizontalPaddle)
            .ok_or(String::from("no paddle"))?;

        if paddle.0 > ball.0 {
            Ok(-1)
        } else if paddle.0 < ball.0 {
            Ok(1)
        } else {
            Ok(0)
        }
    }

    fn next_output(&mut self, x: i64) {
        self.parser.next_output(x)
    }
}

fn main() {
    let mut program_input = String::new();
    File::open("data/day13.in")
        .unwrap()
        .read_to_string(&mut program_input)
        .unwrap();
    let program = parse_intcode_program(&program_input);
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
    // let mut game_input = GameInput { board: &board };
    let mut game_input = AiInput { parser: Parser::new() };
    intcode::run_program(prog, &mut game_input).unwrap();
    println!("{}", game_input.parser.score.unwrap());
}
