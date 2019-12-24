use aoc2019::grid::{Grid, GridBuilder};
use aoc2019::io::slurp_stdin;
use std::collections::HashMap;

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

fn grow_square(current: i64, neighbours: i64) -> i64 {
    if current == BUG && neighbours != 1 {
        EMPTY
    } else if current == EMPTY && (neighbours == 1 || neighbours == 2) {
        BUG
    } else {
        current
    }
}

fn evolve(map: &Map) -> Map {
    let mut b = GridBuilder::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let neighbours = map.get(x+1, y) + map.get(x-1, y) + map.get(x, y+1) + map.get(x, y-1);
            b.push(grow_square(*map.get(x,y), neighbours));
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

struct HyperMap {
    maps: HashMap<i64, Map>,
}

const W: i64 = 5;
const H: i64 = 5;

type Dir = aoc2019::dir::CartesianDir;

fn get_inner(map: &Map, dir: Dir) -> i64 {
    match dir {
        Dir::West => *map.get(1, 2),
        Dir::East => *map.get(3, 2),
        Dir::North => *map.get(2, 1),
        Dir::South => *map.get(2, 3),
    }
}

fn get_outer(map: &Map, dir: Dir) -> i64 {
    let (x, y) = match dir {
        Dir::West => (0,-1),
        Dir::East => (W-1,-1),
        Dir::North => (-1,0),
        Dir::South => (-1,H-1),
    };
    let mut sum = 0;
    if x == -1 {
        for x in 0..W {
            sum += map.get(x, y);
        }
    } else if y == -1 {
        for y in 0..H {
            sum += map.get(x, y);
        }
    }
    sum
}

fn create_empty_map() -> Map {
    let mut elems = Vec::new();
    elems.resize((W*H) as usize, EMPTY);
    Map::new(elems, W as usize, EMPTY)
}

fn hyper_evolve(hyper_map: &HyperMap) -> HyperMap {
    let mut new_maps = HashMap::new();
    let empty_map = create_empty_map();

    for (level, map) in &hyper_map.maps {
        let inner_neighbour = hyper_map.maps.get(&(level+1)).unwrap_or(&empty_map);
        let outer_neighbour = hyper_map.maps.get(&(level-1)).unwrap_or(&empty_map);

        assert_eq!(map.width(), W);
        assert_eq!(map.height(), H);
        let mut b = GridBuilder::new();
        for y in 0..H {
            for x in 0..W {
                if x == 2 && y == 2 {
                    b.push(EMPTY);
                    continue;
                }

                let mut neighbours = map.get(x+1, y) + map.get(x-1, y) + map.get(x, y+1) + map.get(x, y-1);
                if x == 0 {
                    neighbours += get_inner(outer_neighbour, Dir::West);
                }
                if x == W-1 {
                    neighbours += get_inner(outer_neighbour, Dir::East);
                }
                if y == 0 {
                    neighbours += get_inner(outer_neighbour, Dir::North);
                }
                if y == H-1 {
                    neighbours += get_inner(outer_neighbour, Dir::South);
                }

                if x == 2 && y == 1 {
                    neighbours += get_outer(inner_neighbour, Dir::North);
                } else if x == 2 && y == 3 {
                    neighbours += get_outer(inner_neighbour, Dir::South);
                } else if y == 2 && x == 1 {
                    neighbours += get_outer(inner_neighbour, Dir::West);
                } else if y == 2 && x == 3 {
                    neighbours += get_outer(inner_neighbour, Dir::East);
                }

                b.push(grow_square(*map.get(x,y), neighbours));
            }
            b.eol();
        }
        new_maps.insert(*level, b.build(EMPTY));
    }

    HyperMap { maps: new_maps }
}

fn make_hypermap(map: Map, extra_levels: i64) -> HyperMap {
    let mut maps = HashMap::new();
    for level in -extra_levels..=extra_levels {
        maps.insert(level, create_empty_map());
    }
    maps.insert(0, map);
    HyperMap { maps }
}

fn count_bugs(hypermap: &HyperMap) -> i64 {
    hypermap.maps.values()
        .map(|m| {
            m.iter().sum::<i64>()
        })
        .sum::<i64>()
}

fn main() {
    let map = parse_input(&slurp_stdin());

    let (prefix, _cycle) = find_cycle(do_evolve, MapWrapper::new(map.clone()));

    let mut m = map.clone();
    for _ in 0..prefix {
        m = evolve(&m);
    }
    println!("{}", biodiversity_rating(&m));

    let mut hyper_map = make_hypermap(map, 101);
    for _ in 0..200 {
        hyper_map = hyper_evolve(&hyper_map);
    }

    println!("{}", count_bugs(&hyper_map));
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
        let f = |_x: i64| 1;
        // (0) 1 1 1 1 1
        assert_eq!(find_cycle(f, 0), (1, 1));
    }

    #[test]
    fn test_cycle3() {
        let f = |_x: i64| 0;
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

    #[test]
    fn test_day24_example() {
        let input = "....#\n\
                           #..#.\n\
                           #..##\n\
                           ..#..\n\
                           #....";
        let map = parse_input(input);
        let mut hypermap = make_hypermap(map, 5);
        for _ in 0..10 {
            hypermap = hyper_evolve(&hypermap);
        }

        assert_eq!(count_bugs(&hypermap), 99);
    }
}