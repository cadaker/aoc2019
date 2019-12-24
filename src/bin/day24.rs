use aoc2019::grid::{Grid, GridBuilder};
use aoc2019::io::slurp_stdin;

type Map = Grid<i64>;

const BUG: i64 = 1;
const EMPTY: i64 = 0;

fn parse_input(s: &str) -> Map {
    let mut b = GridBuilder::new();
    for c in s.chars() {
        if c == '\n' {
            b.eol();
        } else if c == '#' {
            b.push(BUG);
        } else {
            assert_eq!(c, '.');
            b.push(EMPTY);
        }
    }
    b.build(EMPTY)
}

fn evolve(map: &Map) -> Map {
    let mut b = GridBuilder::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let neighbours = map.get(x+1, y) + map.get(x-1, y) + map.get(x, y+1) + map.get(x, y-1);
            if *map.get(x,y) == BUG && neighbours != 1 {
                b.push(EMPTY);
            } else if *map.get(x,y) == EMPTY && (neighbours == 1 || neighbours == 2) {
                b.push(BUG);
            } else {
                b.push(*map.get(x,y));
            }
        }
        b.eol();
    }
    b.build(EMPTY)
}

fn find_cycle<T: Clone + PartialEq, F: Fn(T) -> T>(f: F, start: T) -> (usize, usize) {
    // Return P, C - where P is the length of the non-repeating prefix, and C
    // the length of the cycle.
    let mut x = f(start.clone());
    let mut x2 = f(f(start.clone()));
    while x != x2 {
        x = f(x);
        x2 = f(f(x2));
    }

    // f^n(x) = f^2n(x)
    // n == P + X, 2n == P + X + k*C
    // ==> n = k*C
    // f(P) == f(P + k*C) == f(n + P)
    let mut y = start;
    let mut p = 0;
    while y != x {
        y = f(y);
        x = f(x);
        p += 1;
    }

    let mut c = 1;
    x = f(x);
    while x != y {
        x = f(x);
        c += 1;
    }

    (p,c)
}

fn biodiversity_rating(map: &Map) -> i64 {
    map.iter()
        .enumerate()
        .map(|(i, b)| {
            b * (1 << (i as i64))
        })
        .sum()
}

#[derive(Clone)]
struct MapWrapper {
    map: Map,
    biodiv: i64,
}

impl MapWrapper {
    fn new(map: Map) -> Self {
        let biodiv = biodiversity_rating(&map);
        MapWrapper { map, biodiv }
    }
}

impl PartialEq for MapWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.biodiv.eq(&other.biodiv)
    }
}

fn do_evolve(mapw: MapWrapper) -> MapWrapper {
    MapWrapper::new(evolve(&mapw.map))
}

fn main() {
    let map = parse_input(&slurp_stdin());

    let (prefix, _cycle) = find_cycle(do_evolve, MapWrapper::new(map.clone()));

    let mut m = map.clone();
    for _ in 0..prefix {
        m = evolve(&m);
    }
    println!("{}", biodiversity_rating(&m));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle1() {
        let f = |x: i64| {
            if x < 3 {
                x+1
            } else {
                3 + (x % 4)
            }
        };
        // (0) 1 2  3 6 5 4  3 6 5 4
        assert_eq!(find_cycle(f, 0), (3, 4));
    }

    #[test]
    fn test_cycle2() {
        let f = |x: i64| 1;
        // (0) 1 1 1 1 1
        assert_eq!(find_cycle(f, 0), (1, 1));
    }

    #[test]
    fn test_cycle3() {
        let f = |x: i64| 0;
        // (0) 0 0 0
        assert_eq!(find_cycle(f, 0), (0, 1));
    }

    #[test]
    fn test_cycle4() {
        let f = |x: i64| (x+1) % 7;
        // (0) 1 2 3 4 5 6  0 1 2 3 4 5 6
        assert_eq!(find_cycle(f, 0), (0, 7));
    }

    #[test]
    fn test_biodiv() {
        let input = ".....\n\
                           .....\n\
                           .....\n\
                           #....\n\
                           .#...";
        assert_eq!(biodiversity_rating(&parse_input(input)), 2129920);
    }
}