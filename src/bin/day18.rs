use aoc2019::io::slurp_stdin;

type Point = (i64, i64);

#[derive(Eq, PartialEq, Clone, Copy)]
enum Elem {
    Open,
    Wall,
    Start,
    Key(usize),
    Door(usize),
}

type Map = aoc2019::grid::Grid<Elem>;

fn read_input(input: &str) -> Map {
    let mut builder =  aoc2019::grid::GridBuilder::new();
    for c in input.chars() {
        match c {
            '#' => builder.push(Elem::Wall),
            '.' => builder.push(Elem::Open),
            '@' => builder.push(Elem::Start),
            'a'..='z' => builder.push(Elem::Key(c as usize - 'a' as usize)),
            'A'..='Z' => builder.push(Elem::Door(c as usize - 'A' as usize)),
            '\n' => builder.eol(),
            _ => unreachable!(),
        }
    }
    builder.build(Elem::Wall)
}

#[derive(Eq, PartialEq, PartialOrd, Ord, Hash, Clone, Copy)]
struct KeySet {
    keys: usize,
}

impl KeySet {
    fn new() -> Self {
        KeySet { keys: 0 }
    }

    fn has_key(&self, key: usize) -> bool {
        self.keys & (1 << key) != 0
    }

    fn set_key(&mut self, key: usize) {
        self.keys |= 1 << key
    }

    /*
    fn clear_key(&mut self, key: usize) {
        self.keys &= !(1 << key)
    }
    */
}

fn union(keys1: &KeySet, keys2: &KeySet) -> KeySet {
    KeySet { keys: keys1.keys | keys2.keys }
}

fn neighbours(p: Point) -> Vec<Point> {
    vec![(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 - 1), (p.0, p.1 + 1)]
}

fn can_move(map: &Map, held_keys: &KeySet, pos: Point) -> (bool, KeySet) {
    match *map.get(pos.0, pos.1) {
        Elem::Wall => (false, *held_keys),
        Elem::Door(key) => (held_keys.has_key(key), *held_keys),
        Elem::Key(key) => {
            let mut new_held_keys = held_keys.clone();
            new_held_keys.set_key(key);
            (true, new_held_keys)
        },
        _ => (true, *held_keys),
    }
}

fn extract_key(map: &Map, pos: Point) -> Option<usize> {
    match *map.get(pos.0, pos.1) {
        Elem::Key(key) => Some(key),
        _ => None,
    }
}

fn reachable(map: &Map, held_keys: &KeySet, start_pos: Point) -> Vec<(Point, usize)> {
    let mut queue = std::collections::VecDeque::<(Point, usize)>::new();
    let mut visited = std::collections::HashSet::<Point>::new();
    let mut ret = Vec::new();

    queue.push_back((start_pos, 0));
    visited.insert(start_pos);

    while !queue.is_empty() {
        let (pos, dist) = queue.pop_front().unwrap();

        let maybe_key = extract_key(map, pos);
        if maybe_key.is_some() && !held_keys.has_key(maybe_key.unwrap()) {
            ret.push((pos, dist));
            continue;
        }

        for n in neighbours(pos) {
            let (valid, _) = can_move(map, held_keys, n);
            if valid && !visited.contains(&n) {
                queue.push_back((n, dist + 1));
                visited.insert(n);
            }
        }
    }
    ret
}

struct SimpleSearch<'a> {
    map: &'a Map,
    all_keys: KeySet,
}

impl aoc2019::dijkstra::Dijkstra for SimpleSearch<'_> {
    type Node = (Point, KeySet);

    fn reachable(&mut self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
        let mut ret = Vec::new();
        for (p, dist) in reachable(self.map, &node.1, node.0) {
            let (valid, held_keys) = can_move(self.map, &node.1, p);
            if valid {
                ret.push(((p, held_keys), dist));
            }
        }
        ret
    }

    fn target(&mut self, node: &Self::Node) -> bool {
        node.1 == self.all_keys
    }
}

fn search(map: &Map, start_pos: Point, all_keys: &KeySet) -> Option<usize> {
    let mut handler = SimpleSearch { map, all_keys: *all_keys };
    let ret = aoc2019::dijkstra::dijkstra(&mut handler, (start_pos, KeySet::new()));
    ret.map(|(_, dist)| dist)
}

fn can_multi_move(map: &Map, held_keys: &KeySet, pos: (Point, Point, Point, Point)) -> (bool, KeySet) {
    let (valid0, keys0) = can_move(map, held_keys, pos.0);
    let (valid1, keys1) = can_move(map, held_keys, pos.1);
    let (valid2, keys2) = can_move(map, held_keys, pos.2);
    let (valid3, keys3) = can_move(map, held_keys, pos.3);
    (valid0 && valid1 && valid2 && valid3,
     union(&union(&keys0, &keys1),
           &union(&keys2, &keys3)))
}

fn multi_reachable(map: &Map, held_keys: &KeySet, start_pos: (Point, Point, Point, Point)) -> Vec<((Point, Point, Point, Point), usize)> {
    let mut ret = Vec::new();
    for (p, dist) in reachable(map, held_keys, start_pos.0) {
        ret.push(((p, start_pos.1, start_pos.2, start_pos.3), dist));
    }
    for (p, dist) in reachable(map, held_keys, start_pos.1) {
        ret.push(((start_pos.0, p, start_pos.2, start_pos.3), dist));
    }
    for (p, dist) in reachable(map, held_keys, start_pos.2) {
        ret.push(((start_pos.0, start_pos.1, p, start_pos.3), dist));
    }
    for (p, dist) in reachable(map, held_keys, start_pos.3) {
        ret.push(((start_pos.0, start_pos.1, start_pos.2, p), dist));
    }
    ret
}

struct MultiSearch<'a> {
    map: &'a Map,
    all_keys: KeySet,
}

impl aoc2019::dijkstra::Dijkstra for MultiSearch<'_> {
    type Node = ((Point, Point, Point, Point), KeySet);

    fn reachable(&mut self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
        let mut ret = Vec::new();
        for (pos, extra_dist) in multi_reachable(self.map, &node.1, node.0) {
            let (valid, new_held_keys) = can_multi_move(self.map, &node.1, pos);
            if valid {
                ret.push(((pos, new_held_keys), extra_dist))
            }
        }
        ret
    }

    fn target(&mut self, node: &Self::Node) -> bool {
        node.1 == self.all_keys
    }
}

fn multi_search(map: &Map, start_pos: (Point, Point, Point, Point), all_keys: &KeySet) -> Option<usize> {
    let mut handler = MultiSearch { map, all_keys: *all_keys };
    let ret = aoc2019::dijkstra::dijkstra(&mut handler, (start_pos, KeySet::new()));
    ret.map(|(_, dist)| dist)
}

fn make_multi_map(map: &Map) -> (Map, (Point, Point, Point, Point)) {
    let pos = map.find_first(&Elem::Start).unwrap();
    let w = map.width();
    let mut elems = map.clone().sink_elems();
    elems[((pos.0 + 0) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 1) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 - 1) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 0) + (pos.1 + 1) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 0) + (pos.1 - 1) * w) as usize] = Elem::Wall;
    let multi_map = Map::new(elems, w as usize, Elem::Wall);
    (multi_map, ((pos.0 + 1, pos.1 + 1),
                 (pos.0 - 1, pos.1 + 1),
                 (pos.0 + 1, pos.1 - 1),
                 (pos.0 - 1, pos.1 - 1)))
}

fn find_all_keys(map: &Map) -> KeySet {
    fn is_key(e: &Elem) -> bool {
        match *e {
            Elem::Key(_) => true,
            _ => false,
        }
    }
    let positions = map.find_all_if(is_key);
    let mut all_keys = KeySet::new();
    for key in 0..positions.len() {
        all_keys.set_key(key);
    }
    all_keys
}

fn do_search(map: &Map) -> Option<usize> {
    let start_pos = map.find_first(&Elem::Start).unwrap();
    let all_keys = find_all_keys(&map);

    search(map, start_pos, &all_keys)
}

fn do_multi_search(map: &Map) -> Option<usize> {
    let (multi_map, start_pos) = make_multi_map(map);
    let all_keys = find_all_keys(map);

    multi_search(&multi_map, start_pos, &all_keys)
}

fn main() {
    let map = read_input(&slurp_stdin());

    let best = do_search(&map).unwrap();

    println!("{}", best);

    let multi_best = do_multi_search(&map).unwrap();
    println!("{}", multi_best);
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ex1() {
        let input = "#########
#b.A.@.a#
#########";
        assert_eq!(do_search(&read_input(&input)), Some(8));
    }

    #[test]
    fn ex2() {
        let input = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";
        assert_eq!(do_search(&read_input(&input)), Some(86));
    }

    #[test]
    fn ex3() {
        let input = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";
        assert_eq!(do_search(&read_input(&input)), Some(132));
    }

/*
    #[test]
    fn ex4() {
        let input = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";
        assert_eq!(do_search(&read_input(&input)), Some(136));
    }
*/

    #[test]
    fn ex5() {
        let input = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";
        assert_eq!(do_search(&read_input(&input)), Some(81));
    }
}
