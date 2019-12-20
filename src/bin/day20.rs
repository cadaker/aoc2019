use aoc2019::grid::{Grid,GridBuilder};
use aoc2019::io::slurp_stdin;
use aoc2019::dijkstra;

#[derive(Eq, PartialEq, Ord, PartialOrd, Copy, Clone)]
enum Elem {
    Wall,
    Open,
    Portal((i64,i64)),
    Start,
    End,
}

type Point = (i64, i64);
type PortalName = (char, char);

struct Portals {
    portals: Vec<(PortalName, Point)>,
}

impl Portals {
    fn find_by_pos(&self, pos: Point) -> Option<PortalName> {
        for (pname, ppos) in &self.portals {
            if pos == *ppos {
                return Some(*pname)
            }
        }
        None
    }

    fn find_other(&self, name: PortalName, pos: Point) -> Option<Point> {
        for (pname, ppos) in &self.portals {
            if *pname == name && *ppos != pos {
                return Some(*ppos);
            }
        }
        None
    }
}

fn find_portals(grid: &Grid<char>) -> Portals {
    fn is_portal_part(c: char) -> bool {
        'A' <= c && c <= 'Z'
    }

    let midx = grid.width() / 2;
    let midy = grid.height() / 2;

    let mut portals = Portals { portals: Vec::new() };

    for y in 0..grid.height() {
        for x in 0..grid.width() {
            // Vertical
            let c0 = *grid.get(x, y);
            let c1 = *grid.get(x,y + 1);
            if is_portal_part(c0) && is_portal_part(c1) {
                if y == 0 {
                    // Outer top
                    portals.portals.push(((c0, c1), (x, y + 2)));
                } else if y == grid.height() - 2 {
                    // Outer bottom
                    portals.portals.push(((c0, c1), (x, y - 1)));
                } else if y < midy {
                    // Inner top
                    portals.portals.push(((c0, c1), (x, y - 1)));
                } else {
                    // Inner bottom
                    portals.portals.push(((c0, c1), (x, y + 2)));
                }
            }
            // Horizontal
            let c0 = *grid.get(x, y);
            let c1 = *grid.get(x + 1,y);
            if is_portal_part(c0) && is_portal_part(c1) {
                if x == 0 {
                    // Outer left
                    portals.portals.push(((c0, c1), (x + 2, y)));
                } else if x == grid.width() - 2 {
                    // Outer right
                    portals.portals.push(((c0, c1), (x - 1, y)));
                } else if x < midx {
                    // Inner left
                    portals.portals.push(((c0, c1), (x - 1, y)));
                } else {
                    // Inner right
                    portals.portals.push(((c0, c1), (x + 2, y)));
                }
            }
        }
    }
    portals
}

fn parse_input(s: &str) -> Grid<Elem> {
    let mut cbuilder = GridBuilder::new();
    for c in s.chars() {
        match c {
            '\n' => cbuilder.eol(),
            _ => cbuilder.push(c),
        }
    }
    let cgrid = cbuilder.build(' ');

    let portals = find_portals(&cgrid);

    let mut builder = GridBuilder::new();
    for y in 2..cgrid.height() - 2 {
        for x in 2..cgrid.width() - 2 {
            let c = *cgrid.get(x, y);
            let elem = match portals.find_by_pos((x, y)) {
                Some(('A', 'A')) => Elem::Start,
                Some(('Z', 'Z')) => Elem::End,
                Some(name) => {
                    let (other_x, other_y) = portals.find_other(name, (x,  y)).unwrap();
                    Elem::Portal((other_x - 2, other_y - 2))
                },
                None => if c == '.' {
                    Elem::Open
                } else {
                    Elem::Wall
                },
            };
            builder.push(elem)
        }
        builder.eol();
    }
    builder.build(Elem::Wall)
}

struct PathFinding<'a> {
    maze: &'a Grid<Elem>,
}

fn neighbours(p: Point) -> Vec<Point> {
    vec![(p.0 - 1, p.1), (p.0 + 1, p.1), (p.0, p.1 - 1), (p.0, p.1 + 1)]
}

impl dijkstra::Dijkstra for PathFinding<'_> {
    type Node = Point;

    fn reachable(&mut self, node: &Self::Node) -> Vec<(Self::Node, usize)> {
        let mut ret = Vec::new();
        for p in neighbours(*node) {
            if *self.maze.get_xy(p) != Elem::Wall {
                ret.push((p, 1));
            }
        }
        match self.maze.get_xy(*node) {
            Elem::Portal(p) => ret.push((*p, 1)),
            _ => (),
        }
        ret
    }

    fn target(&mut self, node: &Self::Node) -> bool {
        *self.maze.get_xy(*node) == Elem::End
    }
}

fn find_path(maze: &Grid<Elem>) -> Option<usize> {
    let start_pos = maze.find_first(&Elem::Start).unwrap();
    let mut dijkstra_handler = PathFinding { maze: &maze };
    let res = dijkstra::dijkstra(&mut dijkstra_handler, start_pos);
    res.map(|r| r.1)
}

fn main() {
    let maze = parse_input(&slurp_stdin());

    let dist = find_path(&maze).unwrap();
    println!("{}", dist);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fix_input(s: &str) -> String {
        let mut lines: Vec<&str> = s.lines().collect();
        let max_width = lines.iter().map(|s| s.len()).max().unwrap();
        let mut ret = String::new();
        for line_str in lines.into_iter() {
            let mut line = String::from(line_str);
            while line.len() < max_width {
                line.push(' ');
            }
            line.push('\n');
            ret.push_str(&line);
        }
        ret
    }

    #[test]
    fn day20_example2() {
        let input = r"                   A
                   A
  #################.#############
  #.#...#...................#.#.#
  #.#.#.###.###.###.#########.#.#
  #.#.#.......#...#.....#.#.#...#
  #.#########.###.#####.#.#.###.#
  #.............#.#.....#.......#
  ###.###########.###.#####.#.#.#
  #.....#        A   C    #.#.#.#
  #######        S   P    #####.#
  #.#...#                 #......VT
  #.#.#.#                 #.#####
  #...#.#               YN....#.#
  #.###.#                 #####.#
DI....#.#                 #.....#
  #####.#                 #.###.#
ZZ......#               QG....#..AS
  ###.###                 #######
JO..#.#.#                 #.....#
  #.#.#.#                 ###.#.#
  #...#..DI             BU....#..LF
  #####.#                 #.#####
YN......#               VT..#....QG
  #.###.#                 #.###.#
  #.#...#                 #.....#
  ###.###    J L     J    #.#.###
  #.....#    O F     P    #.#...#
  #.###.#####.#.#####.#####.###.#
  #...#.#.#...#.....#.....#.#...#
  #.#####.###.###.#.#.#########.#
  #...#.#.....#...#.#.#.#.....#.#
  #.###.#####.###.###.#.#.#######
  #.#.........#...#.............#
  #########.###.###.#############
           B   J   C
           U   P   P               ";
        assert_eq!(find_path(&parse_input(&fix_input(input))), Some(58))
    }
}