use aoc2019::io::slurp_stdin;

type Point = (i32, i32);

#[derive(Eq, PartialEq, Clone, Copy)]
enum Elem {
    Open,
    Wall,
    Start,
    Key(usize),
    Door(usize),
}

struct Map {
    elems: Vec<Elem>,
    elems_width: i32,
}

impl Map {
    fn new(elems: Vec<Elem>, elems_width: i32) -> Self {
        assert!(elems_width > 0);
        assert_eq!(elems.len() as i32 % elems_width, 0);
        Map { elems, elems_width }
    }

    fn width(&self) -> i32 {
        self.elems_width
    }

    fn height(&self) -> i32 {
        self.elems.len() as i32 / self.elems_width
    }

    fn get(&self, x: i32, y: i32) -> Elem {
        if 0 <= x && x < self.width() && 0 <= y && y < self.height() {
            self.elems[(x + y * self.width()) as usize]
        } else {
            Elem::Wall
        }
    }
}

struct MapBuilder {
    elems: Vec<Elem>,
    width: Option<i32>,
}

impl MapBuilder {
    fn new() -> Self {
        MapBuilder { elems: Vec::new(), width: None }
    }

    fn push(&mut self, c: char) {
        match c {
            '#' => self.elems.push(Elem::Wall),
            '.' => self.elems.push(Elem::Open),
            '@' => self.elems.push(Elem::Start),
            'a'..='z' => self.elems.push(Elem::Key(c as usize - 'a' as usize)),
            'A'..='Z' => self.elems.push(Elem::Door(c as usize - 'A' as usize)),
            '\n' => {
                if self.width.is_none() {
                    self.width = Some(self.elems.len() as i32);
                } else {
                    assert_eq!(self.elems.len() as i32 % self.width.unwrap(), 0);
                }
            },
            _ => unreachable!(),
        }
    }

    fn build(self) -> Map {
        Map::new(self.elems, self.width.unwrap())
    }
}

fn read_input(input: &str) -> Map {
    let mut builder = MapBuilder::new();
    for c in input.chars() {
        builder.push(c);
    }
    builder.build()
}

fn find_elem(map: &Map, elem: Elem) -> Option<Point> {
    for y in 0..map.height() {
        for x in 0..map.width() {
            if map.get(x, y) == elem {
                return Some((x,y));
            }
        }
    }
    None
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
    match map.get(pos.0, pos.1) {
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
    match map.get(pos.0, pos.1) {
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

fn search(map: &Map, start_pos: Point, all_keys: &KeySet) -> Option<usize> {
    type Node = (Point, KeySet);

    let mut prio = aoc2019::prio::Prio::<Node, usize>::new();
    let mut finished = std::collections::HashSet::<Node>::new();

    prio.update((start_pos, KeySet::new()), 0);

    while !prio.is_empty() {
        let ((pos, held_keys), dist) = prio.pop().unwrap();
        //println!("visiting {}, {} (dist {}) with keys {:b}", pos.0, pos.1, dist, held_keys.keys);
        finished.insert((pos, held_keys));
        if held_keys == *all_keys {
            return Some(dist);
        }

        for (n, extra_dist) in reachable(map, &held_keys, pos) {
            let (valid, new_held_keys) = can_move(map, &held_keys, n);
            if valid &&
                !finished.contains(&(n, new_held_keys)) &&
                dist + extra_dist < prio.prio_for(&(n, new_held_keys)).unwrap_or(dist + extra_dist + 1)
            {
                prio.update((n, new_held_keys), dist + extra_dist);
            }
        }
    }

    None
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

fn multi_search(map: &Map, start_pos: (Point, Point, Point, Point), all_keys: &KeySet) -> Option<usize> {
    type Node = ((Point, Point, Point, Point), KeySet);

    let mut prio = aoc2019::prio::Prio::<Node, usize>::new();
    let mut finished = std::collections::HashSet::<Node>::new();

    prio.update((start_pos, KeySet::new()), 0);

    while !prio.is_empty() {
        let ((pos, held_keys), dist) = prio.pop().unwrap();
        //println!("visiting {}, {} (dist {}) with keys {:b}", pos.0, pos.1, dist, held_keys.keys);
        finished.insert((pos, held_keys));
        if held_keys == *all_keys {
            return Some(dist);
        }

        for (n, extra_dist) in multi_reachable(map, &held_keys, pos) {
            let (valid, new_held_keys) = can_multi_move(map, &held_keys, n);
            if valid &&
                !finished.contains(&(n, new_held_keys)) &&
                dist + extra_dist < prio.prio_for(&(n, new_held_keys)).unwrap_or(dist + extra_dist + 1)
            {
                prio.update((n, new_held_keys), dist + extra_dist);
            }
        }
    }

    None
}

fn make_multi_map(map: &Map) -> (Map, (Point, Point, Point, Point)) {
    let pos = find_elem(map, Elem::Start).unwrap();
    let w = map.width();
    let mut elems = map.elems.clone();
    elems[((pos.0 + 0) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 1) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 - 1) + (pos.1 + 0) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 0) + (pos.1 + 1) * w) as usize] = Elem::Wall;
    elems[((pos.0 + 0) + (pos.1 - 1) * w) as usize] = Elem::Wall;
    let multi_map = Map { elems, elems_width: w };
    (multi_map, ((pos.0 + 1, pos.1 + 1),
                 (pos.0 - 1, pos.1 + 1),
                 (pos.0 + 1, pos.1 - 1),
                 (pos.0 - 1, pos.1 - 1)))
}

fn find_all_keys(map: &Map) -> KeySet {
    let mut all_keys = KeySet::new();
    let mut key_count = 0;
    while find_elem(&map, Elem::Key(key_count)).is_some() {
        all_keys.set_key(key_count);
        key_count += 1;
    }
    all_keys
}

fn do_search(map: &Map) -> Option<usize> {
    let start_pos = find_elem(&map, Elem::Start).unwrap();
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
