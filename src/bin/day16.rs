use aoc2019::io::slurp_stdin;
use std::io::Write;

fn parse_input(s: &String) -> Vec<i32> {
    s.trim()
        .chars()
        .map(|c| {
            assert!(c.is_digit(10));
            c.to_string().parse::<i32>().unwrap()
        })
        .collect()
}

fn truncate(x: i32) -> i32 {
    if x > 0 {
        x % 10
    } else {
        (-x) % 10
    }
}

struct IntegralVector {
    v: Vec<i32>,
}

impl IntegralVector {
    fn new(xs: &Vec<i32>) -> Self {
        let mut v = Vec::new();
        let mut sum = 0;
        v.push(0);
        for x in xs.iter() {
            sum += *x;
            v.push(sum);
        }
        IntegralVector { v }
    }

    fn get_sum(&self, first: usize, len: usize) -> i32 {
        fn index(v: &Vec<i32>, ix: usize) -> usize {
            if ix < v.len() {
                ix
            } else {
                v.len() - 1
            }
        }

        let high_ix = index(&self.v, first + len);
        let low_ix = index(&self.v, first);
        return self.v[high_ix] - self.v[low_ix];
    }
}

fn fft(xs: &Vec<i32>) -> Vec<i32> {
    let iv = IntegralVector::new(xs);
    let mut ret = Vec::new();

    for len in 1..=xs.len() {
        let mut acc = 0;
        let mut pos = len - 1;
        while pos < xs.len() {
            acc += iv.get_sum(pos, len);
            acc -= iv.get_sum(pos + 2*len, len);
            pos += 4 * len;
        }
        ret.push(truncate(acc));
    }
    ret
}

fn do_repeat_fft(mut x: Vec<i32>, n: usize, print_progress: bool) -> Vec<i32> {
    for i in 0..n {
        x = fft(&x);
        if print_progress {
            print!("{}", {
                if i % 10 == 9 {
                    "0"
                } else {
                    "."
                }});
            std::io::stdout().flush().unwrap();
        }
    }
    if print_progress {
        println!();
    }
    x
}

fn repeat_fft(x: Vec<i32>, n: usize) -> Vec<i32> {
    do_repeat_fft(x, n, false)
}

fn expand(xs: &Vec<i32>) -> Vec<i32> {
    let mut ret = Vec::new();
    const N: usize = 10000;
    ret.reserve(xs.len() * N);
    for _ in 0..N {
        ret.extend_from_slice(&xs);
    }
    ret
}

fn offset(xs: &Vec<i32>) -> usize {
    let mut ret = 0;
    for d in &xs[0..7] {
        assert!(0 <= *d && *d <= 9);
        ret = ret * 10 + (*d as usize);
    }
    ret
}

fn main() {
    let signal = parse_input(&slurp_stdin());

    let x = repeat_fft(signal.clone(), 100);
    for d in &x[0..8] {
        print!("{}", d);
    }
    println!();

    let ex_signal = expand(&signal);
    let n = offset(&ex_signal);
    let ex_fft = do_repeat_fft(ex_signal, 100, true);
    for d in &ex_fft[n..n+8] {
        print!("{}", *d);
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
    fn test_integral_vector() {
        let iv = IntegralVector::new(&vec![1, 2, 3, 4, 5]);
        assert_eq!(iv.get_sum(0, 1), 1);
        assert_eq!(iv.get_sum(0, 2), 3);
        assert_eq!(iv.get_sum(0, 7), 15);
        assert_eq!(iv.get_sum(1, 1), 2);
        assert_eq!(iv.get_sum(1, 3), 9);
        assert_eq!(iv.get_sum(19, 26), 0);
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
