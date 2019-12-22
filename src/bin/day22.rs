use std::collections::VecDeque;
use std::iter::FromIterator;
use aoc2019::io::slurp_stdin;

extern crate regex;

enum Technique {
    DealIntoNew,
    Cut(i64),
    DealWithIncrement(i64),
}

struct Deck {
    // Front is top
    cards: VecDeque<i64>,
}

impl Deck {
    fn new(n: i64) -> Self {
        let mut cards = VecDeque::new();
        for i in 0..n {
            cards.push_back(i)
        }
        Deck { cards }
    }

    fn deal_into_new(&mut self) {
        let mut cards = VecDeque::new();
        for i in self.cards.iter() {
            cards.push_front(*i);
        }
        self.cards = cards;
    }

    fn cut(&mut self, n: i64) {
        let count = if n >= 0 {
            n
        } else {
            self.cards.len() as i64 + n
        };
        for _ in 0..count {
            let i = self.cards.pop_front().unwrap();
            self.cards.push_back(i);
        }
    }

    fn deal_with_increment(&mut self, n: i64) {
        let mut cards = Vec::new();
        cards.resize(self.cards.len(), -1);

        let mut ix = 0i64;
        for card in self.cards.iter() {
            assert_eq!(cards[ix as usize], -1);
            cards[ix as usize] = *card;
            ix = (ix + n) % cards.len() as i64;
        }
        self.cards = VecDeque::from_iter(cards.into_iter());
    }

    fn cards(&self) -> Vec<i64> {
        Vec::from_iter(self.cards.iter().cloned())
    }
}

fn parse_input(s: &str) -> Vec<Technique> {
    let deal_into_new = regex::Regex::new(r"deal into new").unwrap();
    let cut = regex::Regex::new(r"cut (-?[0-9]+)").unwrap();
    let deal_with_inc = regex::Regex::new(r"deal with increment ([0-9]+)").unwrap();

    let mut ret = Vec::new();
    for line in s.lines() {
        if let Some(_) = deal_into_new.captures(line) {
            ret.push(Technique::DealIntoNew)
        } else if let Some(c) = cut.captures(line) {
            ret.push(Technique::Cut(c.get(1).unwrap().as_str().parse().unwrap()));
        } else if let Some(c) = deal_with_inc.captures(line) {
            ret.push(Technique::DealWithIncrement(c.get(1).unwrap().as_str().parse().unwrap()));
        } else {
            assert!(line.is_empty());
        }
    }
    ret
}

// A transform is defined as k*x + m (mod cards)
fn find_transform(techniques: &[Technique], cards: i64) -> (i64, i64) {
    let mut k = 1;
    let mut m = 0;

    for technique in techniques {
        match *technique {
            Technique::DealIntoNew => {
                // x -> C-1 - x (mod C)
                k = -k;
                m = (cards - 1 - m) % cards
            },
            Technique::Cut(n) => {
                // x -> x - n (mod C)
                m = (m - n) % cards
            },
            Technique::DealWithIncrement(n) => {
                // x -> x * n (mod C)
                k = (n * k) % cards;
                m = (n * m) % cards;
            },
        }
    }

    k = (k + cards) % cards;
    m = (m + cards) % cards;
    assert!(0 <= k && k < cards);
    assert!(0 <= m && m < cards);
    (k, m)
}

// base^expt (mod n)
fn exp_modn(base: i64, expt: i64, n: i64) -> i64 {
    let mut base = base as i128;
    let mut expt = expt as i128;
    let n = n as i128;

    assert!(expt >= 0);
    let mut acc = 1i128;
    // Invariant: result = acc * base^expt (mod n)
    while expt > 0 {
        if expt % 2 == 0 {
            base = (base * base) % n;
            expt /= 2;
        } else {
            acc = (acc * base) % n;
            expt -= 1;
        }
    }
    acc as i64
}

fn inverse_mod(x: i64, n: i64) -> Option<i64> {
    assert!(n > 0);
    let mut a = ((x % n) + n) % n;
    let mut a1 = n;
    let mut s = 1;
    let mut s1 = 0;
    let mut t = 0;
    let mut t1 = 1;
    while a1 > 0 {
        let q = a / a1;
        let r = a % a1;
        a = a1;
        a1 = r;
        let ss = s - q*s1;
        s = s1;
        s1 = ss;
        let tt = t - q*t1;
        t = t1;
        t1 = tt;
    }
    assert_eq!((s as i128) * (x as i128) + (t as i128) * (n as i128), a as i128);
    if a == 1 {
        Some(((s % n) + n) % n)
    } else {
        None
    }
}

fn mul_modn(x: i64, y: i64, n: i64) -> i64 {
    (((x as i128) * (y as i128)) % (n as i128)) as i64
}

// If f(x) = k*x + m (mod n), compute f^(expt)(x) (mod n) = k'*x + m' (mod n)
fn exp_transform_modn(k: i64, m: i64, expt: i64, n: i64) -> (i64, i64) {
    let mul = |x, y| mul_modn(x, y, n);

    // k^expt
    let k_expt = exp_modn(k, expt, n);
    // (k-1)^-1
    let km1_inv = inverse_mod(k-1, n).unwrap();

    // 1 + k + k² + k³ + ... + k^n-1 = (k^n - 1) / (k - 1)
    let k_series = mul(k_expt - 1, km1_inv);

    let mm = mul(k_series, m);

    (k_expt, mm)
}

fn main() {
    let techniques = parse_input(&slurp_stdin());

    let mut deck = Deck::new(10007);
    for technique in &techniques {
        match *technique {
            Technique::DealIntoNew => deck.deal_into_new(),
            Technique::Cut(n) => deck.cut(n),
            Technique::DealWithIncrement(n) => deck.deal_with_increment(n),
        }
    }

    for (pos, card) in deck.cards().into_iter().enumerate() {
        if card == 2019 {
            println!("{}", pos);
            break;
        }
    }
    const CARDS: i64 = 119315717514047;
    const TIMES: i64 = 101741582076661;
    // Express the full transform as a transform on the indices: f(i) = k*i + m (mod CARDS)
    let (k, m) = find_transform(&techniques, CARDS);

    // Raise that transform to the power TIMES: f^TIMES(i) = kt*i + mt (mod CARDS)
    let (kt, mt) = exp_transform_modn(k, m, TIMES, CARDS);

    // Compute ans such that kt*ans + mt = 2020 (mod CARDS)
    // ans = (2020 - mt) * kt^-1 (mod CARDS)
    let kt_inv = inverse_mod(kt, CARDS).unwrap();
    let ans = mul_modn(2020 - mt, kt_inv, CARDS);
    assert_eq!((mul_modn(kt, ans, CARDS) + mt) % CARDS, 2020);
    println!("{}", (ans + CARDS) % CARDS);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deal() {
        let mut deck = Deck::new(10);
        deck.deal_into_new();
        assert_eq!(deck.cards, vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut() {
        let mut deck = Deck::new(10);
        deck.cut(3);
        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        let mut deck = Deck::new(10);
        deck.cut(-4);
        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_deal_with_increment() {
        let mut deck = Deck::new(10);
        deck.deal_with_increment(3);
        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3]);
    }
}
