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

fn neighbours(p: Point) -> Vec<Point> {
    vec![(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 - 1), (p.0, p.1 + 1)]
}

fn search(map: &Map, start_pos: Point, all_keys: &KeySet) -> Option<usize> {
    type Node = (Point, KeySet);

    let mut prio = aoc2019::prio::Prio::<(Point, KeySet), usize>::new();
    let mut finished = std::collections::HashSet::<Node>::new();

    prio.update((start_pos, KeySet::new()), 0);

    while !prio.is_empty() {
        let ((pos, held_keys), dist) = prio.pop().unwrap();
        //println!("visiting {}, {} (dist {}) with keys {:b}", pos.0, pos.1, dist, held_keys.keys);
        finished.insert((pos, held_keys));
        if held_keys == *all_keys {
            return Some(dist);
        }

        for n in neighbours(pos) {
            let mut new_held_keys = held_keys;
            match map.get(n.0, n.1) {
                Elem::Wall => continue,
                Elem::Door(key) => {
                    if !held_keys.has_key(key) {
                        continue;
                    }
                },
                Elem::Key(key) => {
                    new_held_keys.set_key(key);
                },
                _ => (),
            };
            if finished.contains(&(n, new_held_keys)) {
                continue;
            }
            if dist + 1 < prio.prio_for(&(n, new_held_keys)).unwrap_or(dist + 2) {
                prio.update((n, new_held_keys), dist + 1);
            }
        }
    }

    None
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

fn main() {
    let map = read_input(&slurp_stdin());

    let best = do_search(&map).unwrap();

    println!("{}", best);
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
