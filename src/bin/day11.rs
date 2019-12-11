use std::collections::HashMap;
use aoc2019::io::slurp_stdin;
use aoc2019::intcode;
use std::cell::RefCell;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Color {
    Black,
    White,
}

const BLACK: i64 = 0;
const WHITE: i64 = 1;

#[derive(Clone,Copy,PartialEq,Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Turn {
    Left,
    Right,
}

const LEFT: i64 = 0;
const RIGHT: i64 = 1;

struct Robot {
    x: i32,
    y: i32,
    colors: HashMap<(i32,i32),Color>,
    dir: Direction,
}

impl Robot {
    fn new() -> Self {
        Robot { x: 0, y: 0, colors: HashMap::new(), dir: Direction::Up }
    }

    fn trigger(&mut self, paint: Color, turn: Turn) {
        self.colors.insert((self.x, self.y), paint);
        self.dir = match (self.dir, turn) {
            (Direction::Up, Turn::Left) => Direction::Left,
            (Direction::Up, Turn::Right) => Direction::Right,
            (Direction::Right, Turn::Left) => Direction::Up,
            (Direction::Right, Turn::Right) => Direction::Down,
            (Direction::Down, Turn::Left) => Direction::Right,
            (Direction::Down, Turn::Right) => Direction::Left,
            (Direction::Left, Turn::Left) => Direction::Down,
            (Direction::Left, Turn::Right) => Direction::Up,
        };
        match self.dir {
            Direction::Up => { self.y += 1; },
            Direction::Right => { self.x += 1; },
            Direction::Down => { self.y -= 1; },
            Direction::Left => { self.x -= 1; },
        }
    }
}

struct RobotInput<'a> {
    robot: &'a RefCell<Robot>,
}

struct RobotOutput<'a> {
    paint_instruction: Option<Color>,
    robot: &'a RefCell<Robot>,
}

impl intcode::Input for RobotInput<'_> {
    fn next_input(&mut self) -> Result<i64, String> {
        let robot = self.robot.borrow();
        let pos = (robot.x, robot.y);
        match *robot.colors.get(&pos).unwrap_or(&Color::Black) {
            Color:: Black => Ok(BLACK),
            Color::White => Ok(WHITE),
        }
    }
}

impl intcode::Output for RobotOutput<'_> {
    fn next_output(&mut self, x: i64) {
        if self.paint_instruction.is_none() {
            self.paint_instruction = match x {
                BLACK => Some(Color::Black),
                WHITE => Some(Color::White),
                _ => unimplemented!(),
            };
            return;
        }
        let move_instruction = match x {
            LEFT => Turn::Left,
            RIGHT => Turn::Right,
            _ => unimplemented!(),
        };
        self.robot.borrow_mut().trigger(self.paint_instruction.unwrap(), move_instruction);
        self.paint_instruction = None;
    }
}


fn main() {
    let program: Vec<intcode::Mem> = slurp_stdin()
        .trim()
        .split(",")
        .map(|s| s.parse::<intcode::Mem>().unwrap())
        .collect();

    {
        let robot = RefCell::new(Robot::new());
        let mut input = RobotInput { robot: &robot };
        let mut output = RobotOutput {
            paint_instruction: None,
            robot: &robot,
        };
        intcode::run_program(program.clone(), &mut input, &mut output).unwrap();

        println!("{}", robot.borrow().colors.len());
    }
    {
        let robot = RefCell::new(Robot::new());
        robot.borrow_mut().colors.insert((0,0), Color::White);
        let mut input = RobotInput { robot: &robot };
        let mut output = RobotOutput {
            paint_instruction: None,
            robot: &robot,
        };
        intcode::run_program(program.clone(), &mut input, &mut output).unwrap();

        let points: Vec<(i32,i32)> = robot.borrow().colors
            .iter()
            .filter(|&(_, color)| *color == Color::White )
            .map(|(p,_)| *p)
            .collect();
        let minx = points.iter().map(|p| p.0).min().unwrap();
        let maxx = points.iter().map(|p| p.0).max().unwrap();
        let miny = points.iter().map(|p| p.1).min().unwrap();
        let maxy = points.iter().map(|p| p.1).max().unwrap();
        for y in (miny..=maxy).rev() {
            for x in minx..=maxx {
                let pixel = robot.borrow().colors.get(&(x,y)).cloned();
                if pixel == Some(Color::White) {
                    print!("#");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}
