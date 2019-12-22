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
