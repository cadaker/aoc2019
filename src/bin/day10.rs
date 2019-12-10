use aoc2019::io::slurp_stdin;
use std::cmp::Ordering::Equal;
use std::collections::HashSet;

fn parse_map(map: &str) -> Vec<(i32,i32)> {
    let mut points = Vec::new();
    let mut x = 0;
    let mut y = 0;
    for ch in map.chars() {
        match ch {
            '#' => { points.push((x,y)); x += 1},
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

fn blocked_by_any(rel_points: &[(i32,i32)], p: (i32,i32)) -> bool {
    for blocker in rel_points {
        if blocks(*blocker, p) && *blocker != p {
            return true;
        }
    }
    false
}

fn unblocked_count(rel_points: &[(i32,i32)]) -> usize {
    rel_points.iter()
        .clone()
        .filter(|&p| !blocked_by_any(rel_points, *p))
        .count()
}

fn center_around(asteroids: &[(i32,i32)], p: (i32,i32)) -> Vec<(i32,i32)> {
    asteroids.iter()
        .clone()
        .map(|q| (q.0 - p.0, q.1 - p.1))
        .filter(|q| !(q.0 == 0 && q.1 == 0))
        .collect()
}

fn asteroid_score(asteroids: &[(i32,i32)], p: (i32,i32)) -> usize {
    let rel_points = center_around(asteroids, p);
    unblocked_count(&rel_points)
}

fn find_best_asteroid(asteroids: &[(i32,i32)]) -> (i32,i32) {
    *asteroids.iter()
        .clone()
        .max_by_key(|p| asteroid_score(asteroids, **p))
        .unwrap()
}

fn single_sweep_laser_hits(rel_points: &[(i32,i32)]) -> Vec<(i32,i32)> {
    fn angle(dx: i32, dy: i32) -> f64 {
        let dxf: f64 = dx.into();
        let dyf: f64 = dy.into();
        (-dxf).atan2(dyf)
    }

    let sweep_order: Vec<(i32,i32)> = {
        let mut tmp = Vec::from(rel_points);
        tmp.sort_by(|p, q| {
            angle(p.0, p.1).partial_cmp(&angle(q.0, q.1)).unwrap_or(Equal)
        });
        tmp
    };

    let mut hit = Vec::new();
    for p in sweep_order {
        if !blocked_by_any(rel_points, p) {
            hit.push(p)
        }
    }
    hit
}

fn find_nth_lasered(rel_points: &[(i32,i32)], n: usize) -> Option<(i32,i32)> {
    let mut left_to_laser = n;
    let mut points_remaining = HashSet::new();
    for p in rel_points {
        points_remaining.insert(*p);
    }

    loop {
        let hit = single_sweep_laser_hits(
            &points_remaining.iter().cloned().collect::<Vec<_>>()
        );
        if hit.len() > left_to_laser {
            return Some(hit[left_to_laser]);
        } else if hit.is_empty() {
            return None;
        }
        left_to_laser -= hit.len();
        for p in hit {
            points_remaining.remove(&p);
        }
    }
}

fn main() {
    let asteroids = parse_map(&slurp_stdin());
    let best_asteroid = find_best_asteroid(&asteroids);
    println!("{}", asteroid_score(&asteroids, best_asteroid));
    let rel_points = center_around(&asteroids, best_asteroid);
    let laser200 = find_nth_lasered(&rel_points, 199).unwrap();
    println!("{}", (laser200.0 + best_asteroid.0) * 100 + (laser200.1 + best_asteroid.1));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map() {
        assert_eq!(parse_map("#.#\n.#.\n"), vec![(0,0), (2,0), (1,1)]);
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
        assert_eq!(centered, vec![(-1,-1), (1,-1), (-1,0), (1,0), (0,1)]);
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

    #[test]
    fn test_angle_sweep_order() {
        let asteroids = parse_map("###\n#.#\n###");
        let rel_points = center_around(&asteroids, (1,1));
        let hit = single_sweep_laser_hits(&rel_points);
        assert_eq!(hit, vec![(0,-1), (1,-1), (1,0), (1,1), (0,1), (-1,1), (-1,0), (-1,-1)]);
    }

    #[test]
    fn test_angle_sweep_occlusion() {
        // #G#2#
        // EF134
        // #D.5#
        // CB976
        // #A#8#
        let asteroids = parse_map("#####\n#####\n##.##\n#####\n#####");
        let rel_points = center_around(&asteroids, (2,2));
        let hit = single_sweep_laser_hits(&rel_points);
        assert_eq!(hit, vec![( 0,-1), ( 1,-2), ( 1,-1), ( 2,-1),
                             ( 1, 0), ( 2, 1), ( 1, 1), ( 1, 2),
                             ( 0, 1), (-1, 2), (-1, 1), (-2, 1),
                             (-1, 0), (-2,-1), (-1,-1), (-1,-2)]);

        assert_eq!(find_nth_lasered(&rel_points, 4), Some((1,0)));
        assert_eq!(find_nth_lasered(&rel_points, 16), Some((0,-2)));
        assert_eq!(find_nth_lasered(&rel_points, 200), None);
    }
}
