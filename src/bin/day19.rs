use aoc2019::io::{parse_intcode_program, slurp_stdin};
use aoc2019::intcode;

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let mut builder = aoc2019::grid::GridBuilder::new();
    for y in 0..50 {
        for x in 0..50 {
            let mut out = vec![];
            intcode::run_program_splitio(program.clone(), &mut vec![y,x], &mut out).unwrap();
            let val = out.first().unwrap();
            builder.push(vec!['.','#'][*val as usize]);
        }
        builder.eol();
    }
    let grid = builder.build('.');
    println!("{}", grid.find_all(&'#').len());
    //println!("{}", grid);
}