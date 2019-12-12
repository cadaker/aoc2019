extern crate regex;

use aoc2019::io::slurp_stdin;

#[derive(Clone,Copy)]
struct V {
    x: i64,
    y: i64,
    z: i64,
}

fn get_input() -> Vec<V> {
    let re = regex::Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut ret = Vec::new();

    for m in re.captures_iter(&slurp_stdin()) {
        fn get(c: &regex::Captures, i: usize) -> i64 {
            c.get(i).unwrap().as_str().parse().unwrap()
        }
        ret.push(V {x: get(&m, 1), y: get(&m, 2), z: get(&m, 3)});
    }
    ret
}

fn step(pos: &mut Vec<V>, vel: &mut Vec<V>) {
    for i in 0..pos.len() {
        for j in i+1..pos.len() {
            if pos[i].x > pos[j].x {
                vel[i].x -= 1;
                vel[j].x += 1;
            } else if pos[i].x < pos[j].x {
                vel[i].x += 1;
                vel[j].x -= 1;
            }

            if pos[i].y > pos[j].y {
                vel[i].y -= 1;
                vel[j].y += 1;
            } else if pos[i].y < pos[j].y {
                vel[i].y += 1;
                vel[j].y -= 1;
            }

            if pos[i].z > pos[j].z {
                vel[i].z -= 1;
                vel[j].z += 1;
            } else if pos[i].z < pos[j].z {
                vel[i].z += 1;
                vel[j].z -= 1;
            }
        }
    }

    for i in 0..pos.len() {
        pos[i].x += vel[i].x;
        pos[i].y += vel[i].y;
        pos[i].z += vel[i].z;
    }
}

fn energy(pos: &Vec<V>, vel: &Vec<V>) -> i64 {
    let mut e = 0;

    fn sum_abs(v: &V) -> i64 {
        v.x.abs() + v.y.abs() + v.z.abs()
    }

    for i in 0..pos.len() {
        e += sum_abs(&pos[i]) * sum_abs(&vel[i]);
    }
    e
}

fn main() {
    let mut pos = get_input();
    let mut vel: Vec<V> = Vec::new();
    vel.resize(pos.len(), V { x: 0, y: 0, z: 0 });

    for _ in 0..1000 {
        step(&mut pos, &mut vel);
    }
    println!("{}", energy(&pos, &vel));
}
