use aoc2019::io::{slurp_stdin, parse_intcode_program};
use aoc2019::intcode;

type Point = (i64,i64);

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Terrain {
    Open,
    Wall,
    Unknown,
}

type Map = std::collections::HashMap<Point, Terrain>;

fn lookup(map: &Map, p: Point) -> Terrain {
    *map.get(&p).unwrap_or(&Terrain::Unknown)
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Step {
    North,
    South,
    West,
    East,
}

const ALL_DIRS: [Step; 4] = [Step::North, Step::South, Step::East, Step::West];

impl Into<intcode::Mem> for Step {
    fn into(self) -> i64 {
        match self {
            Step::North => 1,
            Step::South => 2,
            Step::West => 3,
            Step::East => 4,
        }
    }
}

fn step_to(pos: Point, step: Step) -> Point {
    match step {
        Step::North => (pos.0, pos.1 - 1),
        Step::South => (pos.0, pos.1 + 1),
        Step::West => (pos.0 - 1, pos.1),
        Step::East => (pos.0 + 1, pos.1),
    }
}

fn find_nearest_unknown(map: &Map, robot_pos: Point) -> Option<Point> {
    let mut queue = std::collections::VecDeque::<Point>::new();
    let mut visited = std::collections::HashSet::<Point>::new();
    queue.push_back(robot_pos);
    visited.insert(robot_pos);

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        assert_eq!(lookup(map, next), Terrain::Open);
        for step in &ALL_DIRS {
            let n = step_to(next, *step);
            if lookup(map, n) == Terrain::Unknown {
                return Some(n)
            } else if lookup(map, n) == Terrain::Open && !visited.contains(&n) {
                queue.push_back(n);
                visited.insert(n);
            }
        }
    }

    None
}

fn find_path_to(map: &Map, source: Point, dest: Point) -> Option<Vec<Step>> {
    struct Node {
        pos: Point,
        path: Vec<Step>,
    }

    let mut queue = std::collections::VecDeque::<Node>::new();
    let mut visited = std::collections::HashSet::<Point>::new();
    queue.push_back(Node { pos: source, path: vec![] });
    visited.insert(source);

    while !queue.is_empty() {
        let Node { pos, path } = queue.pop_front().unwrap();
        if pos == dest {
            return Some(path);
        }

        for step in &ALL_DIRS {
            let n = step_to(pos, *step);
            if lookup(map, n) != Terrain::Wall && !visited.contains(&n) {
                let mut new_path = path.clone();
                new_path.push(*step);
                let node = Node { pos: n, path: new_path };
                queue.push_back(node);
                visited.insert(n);
            }
        }
    }

    None
}

struct RobotController {
    map: Map,
    robot_pos: Point,
    path: Vec<Step>,
    oxygen_pos: Option<Point>,
    map_explored: bool,
}

impl RobotController {
    fn new() -> Self {
        let mut map = Map::new();
        let robot_pos = (0,0);
        map.insert(robot_pos, Terrain::Open);
        RobotController {
            map,
            robot_pos,
            path: vec![],
            oxygen_pos: None,
            map_explored: false
        }
    }

    fn is_moving(&self) -> bool {
        !self.path.is_empty()
    }

    fn next_step(&self) -> Step {
        assert!(self.is_moving());
        *self.path.last().unwrap()
    }

    fn move_to(&mut self, pos: Point) {
        assert!(self.path.is_empty());
        let path = find_path_to(&self.map, self.robot_pos, pos);
        assert!(path.is_some());
        self.path = path.expect("could not find a path");
        self.path.reverse();
    }

    fn move_anywhere(&mut self) {
        assert!(self.path.is_empty());
        self.path = vec![Step::North];
    }
}

const BONK: intcode::Mem = 0;
const STEP: intcode::Mem = 1;
const TANK: intcode::Mem = 2;

impl intcode::InputOutput for RobotController {
    fn next_input(&mut self) -> Result<i64, String> {
        assert_eq!(lookup(&self.map, self.robot_pos), Terrain::Open);

        if !self.is_moving() {
            match find_nearest_unknown(&self.map, self.robot_pos) {
                Some(pos) => {
                    self.move_to(pos);
                },
                None => {
                    self.map_explored = true;
                    self.move_anywhere();
                },
            }
        }

        assert!(self.is_moving());

        Ok(self.next_step().into())
    }

    fn next_output(&mut self, x: i64) {
        assert!(!self.path.is_empty());
        let step = self.path.pop().unwrap();

        let pos = step_to(self.robot_pos, step);

        if x == BONK {
            assert!(lookup(&self.map, pos) == Terrain::Wall || lookup(&self.map, pos) == Terrain::Unknown);
            self.map.insert(pos, Terrain::Wall);
        } else if x == STEP || x == TANK {
            assert!(lookup(&self.map, pos) == Terrain::Open || lookup(&self.map, pos) == Terrain::Unknown);
            self.robot_pos = pos;
            self.map.insert(pos, Terrain::Open);
            if x == TANK {
                self.oxygen_pos = Some(pos);
            }
        } else {
            panic!("unknown output");
        }
    }
}

#[allow(dead_code)]
fn print_map(map: &Map, robot_pos: Point) {
    let minx = map.keys().map(|p| p.0).min().unwrap();
    let maxx = map.keys().map(|p| p.0).max().unwrap();
    let miny = map.keys().map(|p| p.1).min().unwrap();
    let maxy = map.keys().map(|p| p.1).max().unwrap();

    for y in (miny..=maxy).rev() {
        for x in minx..=maxx {
            if x == 0 && y == 0 {
                print!("0");
            } else if x == robot_pos.0 && y == robot_pos.1 {
                print!("R")
            } else {
                match lookup(map, (x, y)) {
                    Terrain::Open => print!("."),
                    Terrain::Wall => print!("#"),
                    Terrain::Unknown => print!(" "),
                }
            }
        }
        println!();
    }
}

fn fill_with_oxygen(map: &Map, source: Point) -> usize {
    struct Node {
        pos: Point,
        time: usize,
    }

    let mut highest_time_seen = 0;
    let mut queue = std::collections::VecDeque::<Node>::new();
    let mut visited = std::collections::HashSet::<Point>::new();
    queue.push_back(Node { pos: source, time: 0 });
    visited.insert((0,0));

    while !queue.is_empty() {
        let next = queue.pop_front().unwrap();
        highest_time_seen = std::cmp::max(highest_time_seen, next.time);

        assert_eq!(lookup(map, next.pos), Terrain::Open);
        for step in &ALL_DIRS {
            let n = step_to(next.pos, *step);

            if lookup(&map, n) == Terrain::Open && !visited.contains(&n) {
                queue.push_back(Node { pos: n, time: next.time + 1 });
                visited.insert(n);
            }
        }
    }
    highest_time_seen
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let mut mem = intcode::Memory::new(program);
    let mut ip = 0;
    let mut rel_base = 0;
    let mut controller = RobotController::new();
    while !controller.map_explored {
        match intcode::step_program_io(&mut mem, ip, rel_base, &mut controller).unwrap() {
            intcode::StepResult::Continue(ptr, base) => {
                ip = ptr;
                rel_base = base;
            },
            intcode::StepResult::End => break,
        }
    }
    assert!(controller.map_explored);
    assert!(controller.oxygen_pos.is_some());
    let path = find_path_to(&controller.map, (0,0), controller.oxygen_pos.unwrap());
    println!("{}", path.unwrap().len());

    println!("{}", fill_with_oxygen(&controller.map, controller.oxygen_pos.unwrap()));
}
