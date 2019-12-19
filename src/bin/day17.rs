use aoc2019::io::{slurp_stdin, parse_intcode_program};
use aoc2019::intcode;

type Map = aoc2019::grid::Grid<char>;

struct MapBuilder {
    grid_builder: aoc2019::grid::GridBuilder<char>,
}

impl intcode::Output for MapBuilder {
    fn next_output(&mut self, x: i64) {
        let c = x as u8 as char;
        if c == '\n' {
            self.grid_builder.eol();
        } else {
            self.grid_builder.push(c);
        }
    }
}

fn find_intersections(map: &Map) -> Vec<(i64, i64)> {
    fn is_scaffold(c: char) -> bool {
        c == '#' || c == '<' || c == '>' || c == '^' || c == 'v'
    }

    let mut ret = Vec::new();
    for y in 1..map.height() - 1 {
        for x in 1..map.width() - 1 {
            if is_scaffold(*map.get(x, y)) &&
                is_scaffold(*map.get(x - 1, y)) &&
                is_scaffold(*map.get(x + 1, y)) &&
                is_scaffold(*map.get(x, y - 1)) &&
                is_scaffold(*map.get(x, y + 1))
            {
                ret.push((x, y));
            }
        }
    }
    ret
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Into<(i32, i32)> for Dir {
    fn into(self) -> (i32, i32) {
        match self {
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::Up => (0, -1),
            Dir::Down => (0, 1),
        }
    }
}

fn find_robot(map: &Map) -> Option<(i64, i64, Dir)> {
    for y in 0..map.height() {
        for x in 0..map.width() {
            match map.get(x, y) {
                '<' => return Some((x, y, Dir::Left)),
                '>' => return Some((x, y, Dir::Right)),
                '^' => return Some((x, y, Dir::Up)),
                'v' => return Some((x, y, Dir::Down)),
                _ => (),
            }
        }
    }
    None
}

fn can_walk(map: &Map, x: i64, y: i64, dir: Dir) -> bool {
    match dir {
        Dir::Left => x > 0 && *map.get(x - 1, y) != '.',
        Dir::Right => x < map.width() - 1 && *map.get(x + 1, y) != '.',
        Dir::Up => y > 0 && *map.get(x, y - 1) != '.',
        Dir::Down => y < map.height() - 1 && *map.get(x, y + 1) != '.',
    }
}

fn valid_turn(map: &Map, x: i64, y: i64, dir: Dir) -> Option<char> {
    let left = *map.get(x - 1, y);
    let right = *map.get(x + 1, y);
    let up = *map.get(x, y - 1);
    let down = *map.get(x, y + 1);

    let choose = |left_c, right_c| -> Option<char> {
        if left_c != '.' {
            Some('L')
        } else if right_c != '.' {
            Some('R')
        } else {
            None
        }
    };

    match dir {
        Dir::Left => choose(down, up),
        Dir::Right => choose(up, down),
        Dir::Up => choose(left, right),
        Dir::Down => choose(right, left),
    }
}

fn turn(dir: Dir, turning: char) -> Option<Dir> {
    match (dir, turning) {
        (Dir::Left, 'L') => Some(Dir::Down),
        (Dir::Left, 'R') => Some(Dir::Up),
        (Dir::Right, 'L') => Some(Dir::Up),
        (Dir::Right, 'R') => Some(Dir::Down),
        (Dir::Up, 'L') => Some(Dir::Left),
        (Dir::Up, 'R') => Some(Dir::Right),
        (Dir::Down, 'L') => Some(Dir::Right),
        (Dir::Down, 'R') => Some(Dir::Left),
        _ => None,
    }
}

fn greedy_path(map: &Map) -> Vec<String> {
    let (mut x, mut y, mut dir) = find_robot(map).unwrap();
    let mut ret = Vec::new();

    loop {
        let mut steps = 0;
        while can_walk(map, x, y, dir) {
            match dir {
                Dir::Left => x -= 1,
                Dir::Right => x += 1,
                Dir::Up => y -= 1,
                Dir::Down => y += 1,
            }
            steps += 1;
        }
        if steps > 0 {
            ret.push(steps.to_string());
        }
        match valid_turn(map, x, y, dir) {
            Some(c) => {
                dir = turn(dir, c).unwrap();
                ret.push(c.to_string());
            },
            None => return ret,
        }
        assert!(can_walk(map, x, y, dir));
    }
}

fn break_down_commands<'a>(cmds: &'a [String], routines: &mut Vec<&'a [String]>, used: &mut Vec<usize>) -> Option<()> {
    if cmds.is_empty() && used.len() <= 10 {
        return Some(());
    }
    let routines_tmp = routines.clone();
    for (i, r) in routines_tmp.into_iter().enumerate() {
        if r.len() <= cmds.len() && cmds[..r.len()] == r[..] {
            used.push(i);
            let recurse = break_down_commands(&cmds[r.len()..], routines, used);
            if recurse.is_some() {
                return recurse;
            }
            used.pop();
        }
    }
    if routines.len() < 4 {
        for prefix_len in 1..=std::cmp::min(cmds.len(), 10) {
            used.push(routines.len());
            routines.push(&cmds[..prefix_len]);
            let recurse = break_down_commands(&cmds[prefix_len..], routines, used);
            if recurse.is_some() {
                return recurse;
            }
            routines.pop();
            used.pop();
        }
    }
    None
}

fn main() {
    let program = parse_intcode_program(&slurp_stdin());

    let mut map_builder = MapBuilder { grid_builder: aoc2019::grid::GridBuilder::new() };
    intcode::run_program_splitio(program.clone(), &mut vec![], &mut map_builder).unwrap();
    let map = map_builder.grid_builder.build('.');

    let intersections = find_intersections(&map);

    let alignments: i64 = intersections.iter()
        .map(|(x, y)| *x * *y)
        .sum();

    println!("{}", alignments);
    // println!("{}", map);
    let path = greedy_path(&map);

    fn make_string<T: ToString>(v: &Vec<T>) -> String {
        let mut ret = String::new();
        for (i, x) in v.iter().enumerate() {
            if i != 0 {
                ret.push(',');
            }
            ret.push_str(&x.to_string());
        }
        ret.push('\n');
        ret
    }

    let mut routines_ref = vec![];
    let mut used = vec![];
    let res = break_down_commands(&path, &mut routines_ref, &mut used);
    let routines: Vec<Vec<String>> = routines_ref.iter().map(|s| s.to_vec()).collect();

    assert!(res.is_some());

    let mut input_string = String::new();
    input_string.push_str(
        &make_string(&used.into_iter()
            .map(|u| vec!['A', 'B', 'C', 'D'][u])
            .collect()));

    assert!(routines.len() < 4);
    for r in &routines {
        input_string.push_str(&make_string(r));
    }
    for _ in routines.len()..4 {
        input_string.push('\n');
    }

    input_string.push_str("n\n");

    // print!("{}", input_string);

    let mut out = vec![];
    let mut prog = program.clone();
    prog[0] = 2;
    let mut input: Vec<intcode::Mem> = input_string.chars().map(|c| c as intcode::Mem).collect();
    input.reverse();

    intcode::run_program_splitio(prog, &mut input, &mut out).unwrap();
    println!("{}", out.last().unwrap());
}
