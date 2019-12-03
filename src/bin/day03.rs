use std::io::{self, Read};

fn slurp_stdin() -> String {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut buf = String::new();
    handle.read_to_string(&mut buf).expect("read failure");
    buf
}

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

fn trace_path(steps: &[Step]) -> std::collections::HashSet<Point> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut points = std::collections::HashSet::<Point>::new();
    for step in steps {
        let (dx, dy, n) = step_movement(step);
        for _ in 0..n {
            x += dx;
            y += dy;
            points.insert((x,y));
        }
    }
    points
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

    let points0 = trace_path(&path0);
    let points1 = trace_path(&path1);

    let intersection_points: Vec<Point> =
        points0.intersection(&points1)
            .cloned()
            .collect();
    let minnorm = intersection_points
        .iter()
        .cloned()
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
