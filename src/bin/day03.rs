use std::io::{self, Read};

fn slurp_stdin() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");
    buf
}

#[derive(Clone, Copy)]
enum Step {
    Right(i32),
    Left(i32),
    Up(i32),
    Down(i32),
}

fn parse_entry(s: &str) -> Step {
    let n: i32 = s[1..].parse().unwrap();
    match &s[0..1] {
        "R" => Step::Right(n),
        "L" => Step::Left(n),
        "U" => Step::Up(n),
        "D" => Step::Down(n),
        _ => { assert!(false); Step::Right(0) }
    }
}

type Point = (i32, i32);

fn step_movement(step: &Step) -> (i32, i32, i32) {
    match *step {
        Step::Left(n)  => (-1, 0, n),
        Step::Right(n) => (1, 0, n),
        Step::Up(n)    => (0, 1, n),
        Step::Down(n)  => (0, -1, n),
    }
}

struct PathIterator<I> where I: Iterator<Item=Step> {
    x: i32,
    y: i32,
    dx: i32,
    dy: i32,
    n: i32,
    step_iter: I
}

fn path_iter<I>(step_iter: I) -> PathIterator<I> where I: Iterator<Item=Step> {
    PathIterator::<I> { x: 0, y: 0, dx: 0, dy: 0, n: 0, step_iter }
}

impl<I> Iterator for PathIterator<I> where I: Iterator<Item=Step> {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        while self.n <= 0 {
            match self.step_iter.next() {
                Some(s) => {
                    let (dx, dy, n) = step_movement(&s.into());
                    self.dx = dx;
                    self.dy = dy;
                    self.n = n;
                },
                None => return None
            }
        }

        self.x += self.dx;
        self.y += self.dy;
        self.n -= 1;
        Some((self.x,self.y))
    }
}

fn mh_norm(p: &Point) -> i32 {
    p.0.abs() + p.1.abs()
}

fn distance_to(steps: &[Step], destination: Point) -> Option<i32> {
    let mut dist: i32 = 0;
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    for step in steps {
        if (x,y) == destination {
            return Some(dist);
        }
        let (dx,dy,n) = step_movement(step);
        for _ in 0..n {
            x += dx;
            y += dy;
            dist += 1;
            if (x,y) == destination {
                return Some(dist);
            }
        }
    }
    None
}

fn combined_distance(steps0: &[Step], steps1: &[Step], destination: Point) -> i32 {
    let dist0 = distance_to(steps0, destination).unwrap();
    let dist1 = distance_to(steps1, destination).unwrap();
    dist0 + dist1
}

fn main() {
    let lines: Vec<String> = slurp_stdin().split("\n").map(|s| { s.to_string() }).collect();
    let path0: Vec<Step> = lines[0].split(",").map(|s| { parse_entry(s) }).collect();
    let path1: Vec<Step> = lines[1].split(",").map(|s| { parse_entry(s) }).collect();

    let points0: std::collections::HashSet<Point> = path_iter(path0.iter().cloned()).collect();
    let points1: std::collections::HashSet<Point> = path_iter(path1.iter().cloned()).collect();

    let intersection_points: Vec<Point> =
        points0.intersection(&points1)
            .cloned()
            .collect();
    let minnorm = intersection_points
        .iter()
        .map(|p| mh_norm(&p))
        .min()
        .unwrap();
    println!("{}", minnorm);

    let mindist = intersection_points
        .iter()
        .cloned()
        .map(|p| combined_distance(&path0, &path1, p))
        .min()
        .unwrap();
    println!("{}", mindist);
}
