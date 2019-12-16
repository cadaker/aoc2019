use aoc2019::io::slurp_stdin;

fn parse_input(s: &String) -> Vec<i32> {
    s.trim()
        .chars()
        .map(|c| {
            assert!(c.is_digit(10));
            c.to_string().parse::<i32>().unwrap()
        })
        .collect()
}

fn pattern_base(pos: usize) -> Vec<i32> {
    let mut ret = Vec::new();
    for x in &[0, 1, 0, -1] {
        for _ in 0..pos+1 {
            ret.push(*x);
        }
    }
    assert_eq!(ret.len(), (pos + 1)*4);
    ret
}

fn truncate(x: i32) -> i32 {
    if x > 0 {
        x % 10
    } else {
        (-x) % 10
    }
}

struct Pattern {
    base: Vec<i32>,
    pos: usize,
}

impl Pattern {
    fn new(pos: usize) -> Self {
        Pattern { base: pattern_base(pos), pos: 1 }
    }
}

impl Iterator for Pattern {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = self.base[self.pos];
        self.pos = (self.pos + 1) % self.base.len();
        Some(ret)
    }
}

fn fft(xs: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    for pos in 0..xs.len() {
        let pattern = Pattern::new(pos);
        let n = xs.iter().cloned().zip(pattern)
            .map(|(x,p)| p * x)
            .sum();
        ret.push(truncate(n));
    }
    ret
}

fn repeat_fft(mut x: Vec<i32>, n: usize) -> Vec<i32> {
    for _ in 0..n {
        x = fft(&x);
    }
    x
}

fn main() {
    let signal = parse_input(&slurp_stdin());

    let x = repeat_fft(signal.clone(), 100);
    for d in &x[0..8] {
        print!("{}", d);
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_truncate() {
        assert_eq!(truncate(0), 0);
        assert_eq!(truncate(-1), 1);
        assert_eq!(truncate(38), 8);
        assert_eq!(truncate(-17), 7);
    }

    #[test]
    fn test_pattern() {
        assert_eq!(pattern_base(0), vec![0, 1, 0, -1]);
        assert_eq!(pattern_base(1), vec![0, 0, 1, 1, 0, 0, -1, -1]);
        assert_eq!(pattern_base(2), vec![0, 0, 0, 1, 1, 1, 0, 0, 0, -1, -1, -1]);
    }

    #[test]
    fn test_fft() {
        let xs = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let fft1 = fft(&xs);
        let fft2 = fft(&fft1);
        let fft3 = fft(&fft2);
        assert_eq!(fft1, vec![4, 8, 2, 2, 6, 1, 5, 8]);
        assert_eq!(fft2, vec![3, 4, 0, 4, 0, 4, 3, 8]);
        assert_eq!(fft3, vec![0, 3, 4, 1, 5, 5, 1, 8]);
    }

    #[test]
    fn test_examples() {
        let ex1 = parse_input(&String::from("80871224585914546619083218645595"));
        let ex2 = parse_input(&String::from("19617804207202209144916044189917"));
        let ex3 = parse_input(&String::from("69317163492948606335995924319873"));
        assert_eq!(repeat_fft(ex1, 100)[0..8].to_vec(), vec![2, 4, 1, 7, 6, 1, 7, 6]);
        assert_eq!(repeat_fft(ex2, 100)[0..8].to_vec(), vec![7, 3, 7, 4, 5, 4, 1, 8]);
        assert_eq!(repeat_fft(ex3, 100)[0..8].to_vec(), vec![5, 2, 4, 3, 2, 1, 3, 3]);
    }
}
