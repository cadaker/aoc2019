use std::collections::HashSet;
use aoc2019::io::slurp_stdin;

fn parse_map(map: &str) -> HashSet<(i32,i32)> {
    let mut points = HashSet::new();
    let mut x = 0;
    let mut y = 0;
    for ch in map.chars() {
        match ch {
            '#' => { points.insert((x,y)); x += 1},
            '\n' => { y += 1; x = 0; },
            _ => { x += 1; }
        }
    }
    points
}

fn gcd(x: i32, y: i32) -> i32 {
    if x == 0 {
        y
    } else {
        gcd(y%x, x)
    }
}

fn blocks(blocker: (i32,i32), blocked: (i32,i32)) -> bool {
    let blocker_norm = gcd(blocker.0.abs(), blocker.1.abs());
    let blocked_norm = gcd(blocked.0.abs(), blocked.1.abs());
    blocker.0.abs() <= blocked.0.abs() &&
        blocker.1.abs() <= blocked.1.abs() &&
        blocker.0/blocker_norm == blocked.0/blocked_norm &&
        blocker.1/blocker_norm == blocked.1/blocked_norm

}

fn blocked_by_any(rel_points: &HashSet<(i32,i32)>, p: (i32,i32)) -> bool {
    for blocker in rel_points {
        if blocks(*blocker, p) && *blocker != p {
            return true;
        }
    }
    false
}

fn unblocked_count(rel_points: &HashSet<(i32,i32)>) -> usize {
    rel_points.iter()
        .clone()
        .filter(|&p| !blocked_by_any(rel_points, *p))
        .count()
}

fn center_around(asteroids: &HashSet<(i32,i32)>, p: (i32,i32)) -> HashSet<(i32,i32)> {
    asteroids.iter()
        .clone()
        .map(|q| (q.0 - p.0, q.1 - p.1))
        .filter(|q| !(q.0 == 0 && q.1 == 0))
        .collect()
}

fn asteroid_score(asteroids: &HashSet<(i32,i32)>, p: (i32,i32)) -> usize {
    let rel_points = center_around(asteroids, p);
    unblocked_count(&rel_points)
}

fn find_best_asteroid(asteroids: &HashSet<(i32,i32)>) -> usize {
    asteroids.iter()
        .clone()
        .map(|p| asteroid_score(asteroids, *p))
        .max()
        .unwrap()
}

fn main() {
    let asteroids = parse_map(&slurp_stdin());
    println!("{}", find_best_asteroid(&asteroids));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::hash::Hash;

    fn to_set<T: Eq + Hash>(v: Vec<T>) -> HashSet<T> {
        let mut set = HashSet::new();
        for x in v {
            set.insert(x);
        }
        set
    }

    #[test]
    fn test_map() {
        assert_eq!(parse_map("#.#\n.#.\n"), to_set(vec![(0,0), (2,0), (1,1)]));
    }

    #[test]
    fn test_blocks() {
        assert!(blocks((1,2), (2,4)));
        assert!(blocks((-1,2), (-2,4)));
        assert!(blocks((1,-2), (2,-4)));
        assert!(blocks((-1,-2), (-2,-4)));

        assert!(!blocks((-1,2), (2,4)));
        assert!(!blocks((-1,2), (2,-4)));
    }

    #[test]
    fn test_centering() {
        let asteroids = parse_map("#.#\n###\n.#.");
        let centered = center_around(&asteroids, (1,1));
        assert_eq!(centered, to_set(vec![(-1,-1), (1,-1), (-1,0), (1,0), (0,1)]));
    }

    #[test]
    fn test_scores() {
        let asteroids = parse_map(".#..#
.....
#####
....#
...##");
        assert_eq!(asteroid_score(&asteroids, (1,0)), 7);
        assert_eq!(asteroid_score(&asteroids, (4,0)), 7);
        assert_eq!(asteroid_score(&asteroids, (0,2)), 6);
        assert_eq!(asteroid_score(&asteroids, (1,2)), 7);
        assert_eq!(asteroid_score(&asteroids, (2,2)), 7);
        assert_eq!(asteroid_score(&asteroids, (3,2)), 7);
        assert_eq!(asteroid_score(&asteroids, (4,2)), 5);
        assert_eq!(asteroid_score(&asteroids, (4,3)), 7);
        assert_eq!(asteroid_score(&asteroids, (3,4)), 8);
        assert_eq!(asteroid_score(&asteroids, (4,4)), 7);
    }
}