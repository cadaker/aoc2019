use aoc2019::io::{parse_intcode_program, slurp_stdin};
use aoc2019::intcode;

fn scan(program: Vec<intcode::Mem>, x: i64, y: i64) -> bool {
    let mut out = vec![];
    intcode::run_program_splitio(program.clone(), &mut vec![y,x], &mut out).unwrap();
    *out.first().unwrap() > 0
}

fn beam_limits(program: &Vec<intcode::Mem>, y: i64) -> (i64, i64) {
    let mut x = 0;
    let startx = {
        loop {
            if scan(program.clone(), x, y) {
                break x;
            }
            x += 1;
        }
    };
    let endx = {
        loop {
            if !scan(program.clone(), x, y) {
                break x;
            }
            x += 1;
        }
    };
    (startx, endx)
}

fn beam_limits_guess(program: &Vec<intcode::Mem>, y: i64, x0: i64, x1: i64) -> (i64, i64) {
    let mut x = x0;
    let startx = {
        loop {
            if scan(program.clone(), x, y) {
                break x;
            }
            x += 1;
        }
    };
    x = x1;
    let endx = {
        loop {
            if !scan(program.clone(), x, y) {
                break x;
            }
            x += 1;
        }
    };
    (startx, endx)
}

fn find_square(program: &Vec<intcode::Mem>) -> (i64, i64) {
    let mut y = 100;
    let (mut x0, mut x1) = beam_limits(program, y);
    while x1 - x0 < 100 {
        y += 1;
        let xs = beam_limits_guess(program, y, x0, x1);
        x0 = xs.0;
        x1 = xs.1;
    }
    let (mut xx0, mut xx1) = beam_limits_guess(program, y+99, x0, x1);
    while !(xx0+99 < x1) {
        y += 1;
        let p = beam_limits_guess(program, y, x0, x1);
        let pp = beam_limits_guess(program, y+99, xx0, xx1);
        x0 = p.0;
        x1 = p.1;
        xx0 = pp.0;
        xx1 = pp.1;
    }
    (xx0, y)
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let mut builder = aoc2019::grid::GridBuilder::new();
    for y in 0..50 {
        for x in 0..50 {
            let val = scan(program.clone(), x, y);
            builder.push(vec!['.','#'][val as usize]);
        }
        builder.eol();
    }
    let grid = builder.build('.');
    println!("{}", grid.find_all(&'#').len());
    //println!("{}", grid);

    let (x, y) = find_square(&program);
    println!("{}", x * 10000 + y);
}
