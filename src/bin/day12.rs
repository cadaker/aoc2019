extern crate regex;

use aoc2019::io::slurp_stdin;

fn get_input() -> (Vec<i64>, Vec<i64>, Vec<i64>) {
    let re = regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();

    let mut xs = Vec::new();
    let mut ys = Vec::new();
    let mut zs = Vec::new();

    for m in re.captures_iter(&slurp_stdin()) {
        let get = |i| -> i64 {
            m.get(i).unwrap().as_str().parse().unwrap()
        };
        xs.push(get(1));
        ys.push(get(2));
        zs.push(get(3));
    }
    (xs,ys,zs)
}

fn step_coord(pos: &mut [i64], vel: &mut [i64]) {
    for i in 0..pos.len() {
        for j in i+1..pos.len() {
            if pos[i] > pos[j] {
                vel[i] -= 1;
                vel[j] += 1;
            } else if pos[i] < pos[j] {
                vel[i] += 1;
                vel[j] -= 1;
            }
        }
    }

    for i in 0..pos.len() {
        pos[i] += vel[i];
    }
}

fn step(
    pos_x: &mut [i64],
    pos_y: &mut [i64],
    pos_z: &mut [i64],
    vel_x: &mut [i64],
    vel_y: &mut [i64],
    vel_z: &mut [i64])
{
    step_coord(pos_x, vel_x);
    step_coord(pos_y, vel_y);
    step_coord(pos_z, vel_z);
}

fn energy(
    pos_x: &[i64],
    pos_y: &[i64],
    pos_z: &[i64],
    vel_x: &[i64],
    vel_y: &[i64],
    vel_z: &[i64]) -> i64
{
    let mut e = 0;

    fn sum_abs(x: i64, y: i64, z: i64) -> i64 {
        x.abs() + y.abs() + z.abs()
    }

    for i in 0..pos_x.len() {
        e += sum_abs(pos_x[i], pos_y[i], pos_z[i]) * sum_abs(vel_x[i], vel_y[i], vel_z[i]);
    }
    e
}

fn main() {
    let (mut pos_x, mut pos_y, mut pos_z) = get_input();
    let zeros = {
        let mut v = Vec::new();
        v.resize(pos_x.len(), 0i64);
        v
    };
    let mut vel_x = zeros.clone();
    let mut vel_y = zeros.clone();
    let mut vel_z = zeros.clone();

    for _ in 0..1000 {
        step(&mut pos_x, &mut pos_y, &mut pos_z, &mut vel_x, &mut vel_y, &mut vel_z);
    }
    println!("{}", energy(&pos_x, &pos_y, &pos_z, &vel_x, &vel_y, &vel_z));
}
