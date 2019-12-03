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

fn trace(points: &mut std::collections::HashSet<Point>, x: &mut i32, y: &mut i32, dx: i32, dy: i32, n: i32) {
    for _ in 0..n {
        *x += dx;
        *y += dy;
        points.insert((*x,*y));
    }
}

fn trace_path(steps: &[Step]) -> std::collections::HashSet<Point> {
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut points = std::collections::HashSet::<Point>::new();
    for step in steps.iter() {
        match step {
            Step::Left(n)  => trace(&mut points, &mut x, &mut y, -1, 0, *n),
            Step::Right(n) => trace(&mut points, &mut x, &mut y, 1, 0, *n),
            Step::Up(n)    => trace(&mut points, &mut x, &mut y, 0, 1, *n),
            Step::Down(n)  => trace(&mut points, &mut x, &mut y, 0, -1, *n),
        };
    }
    points
}

fn mh_norm(p: &Point) -> i32 {
    p.0.abs() + p.1.abs()
}

fn main() {
    let lines: Vec<String> = slurp_stdin().split("\n").map(|s| { s.to_string() }).collect();
    let path0: Vec<Step> = lines[0].split(",").map(|s| { parse_entry(s) }).collect();
    let path1: Vec<Step> = lines[1].split(",").map(|s| { parse_entry(s) }).collect();

    let points0 = trace_path(&path0);
    let points1 = trace_path(&path1);

    let (xmin, ymin) = points0.intersection(&points1).min_by_key(|p| { mh_norm(*p) }).unwrap();
    println!("{}", mh_norm(&(*xmin, *ymin)))
}
