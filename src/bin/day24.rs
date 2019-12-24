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

fn basic_neighbours(map: &Map, x: i64, y: i64) -> i64 {
    map.get(x+1, y) + map.get(x-1, y) + map.get(x, y+1) + map.get(x, y-1)
}

fn evolve<NeighbourFn: Fn(i64, i64) -> i64>(map: &Map, neighbour_func: NeighbourFn) -> Map {
    let mut b = GridBuilder::new();
    for y in 0..map.height() {
        for x in 0..map.width() {
            let neighbours = neighbour_func(x, y);
            b.push(grow_square(*map.get(x,y), neighbours));
        }
        b.eol();
    }
    b.build(EMPTY)
}

fn biodiversity_rating(map: &Map) -> i64 {
    map.iter()
        .enumerate()
        .map(|(i, b)| {
            b * (1 << (i as i64))
        })
        .sum()
}

fn find_first_duplicate(map: &Map) -> Map {
    let mut found = std::collections::HashSet::new();
    let mut map = map.clone();
    loop {
        let bio = biodiversity_rating(&map);
        if found.contains(&bio) {
            return map;
        } else {
            found.insert(bio);
            map = evolve(&map, |x, y| basic_neighbours(&map, x, y));
        }
    }
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

fn hyper_neighbours(map: &Map, inner_neighbour: &Map, outer_neighbour: &Map, x: i64, y: i64) -> i64 {
    if x == 2 && y == 2 {
        return 0;
    }

    let mut neighbours = basic_neighbours(map, x, y);
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
    neighbours
}

fn hyper_evolve(hyper_map: &HyperMap) -> HyperMap {
    let mut new_maps = HashMap::new();
    let empty_map = create_empty_map();

    for (level, map) in &hyper_map.maps {
        let inner_neighbour = hyper_map.maps.get(&(level+1)).unwrap_or(&empty_map);
        let outer_neighbour = hyper_map.maps.get(&(level-1)).unwrap_or(&empty_map);

        assert_eq!(map.width(), W);
        assert_eq!(map.height(), H);

        let new_map = evolve(map, |x, y| {
            hyper_neighbours(&map, inner_neighbour, outer_neighbour, x, y)
        });

        new_maps.insert(*level, new_map);
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

    let m = find_first_duplicate(&map);
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