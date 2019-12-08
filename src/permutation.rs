fn next_permutation<T: PartialOrd>(elems: &mut Vec<T>) -> bool {
    if elems.len() < 2 {
        return false;
    }

    let smaller_pos = {
        let mut ix = elems.len() - 1;
        while ix > 0 && !(elems[ix-1] < elems[ix]) {
            ix -= 1;
        }
        // all elems[ix] >= elems[ix+1]
        if ix == 0 {
            elems.reverse();
            return false;
        }
        ix -= 1;
        assert!(elems[ix] < elems[ix+1]);
        ix
    };
    let next_larger_pos = {
        let mut ix = elems.len() - 1;
        while !(elems[ix] > elems[smaller_pos]) {
            ix -= 1;
        }
        assert!(ix > smaller_pos);
        ix
    };

    elems.swap(smaller_pos, next_larger_pos);
    elems[smaller_pos+1..].reverse();

    true
}

pub struct Permutations<T: PartialOrd+Clone> {
    elems: Vec<T>,
    done: bool,
}

impl<T: PartialOrd+Clone> Permutations<T> {
    pub fn new(elems: Vec<T>) -> Self {
        Permutations {elems, done: false}
    }
}

impl<T: PartialOrd+Clone> Iterator for Permutations<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            None
        } else {
            let ret = self.elems.clone();
            self.done = !next_permutation(&mut self.elems);
            Some(ret)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_permutation_test() {
        {
            let mut xs = vec![0, 1, 2, 3];
            assert_eq!(next_permutation(&mut xs), true);
            assert_eq!(xs, [0, 1, 3, 2]);
            assert_eq!(next_permutation(&mut xs), true);
            assert_eq!(xs, [0, 2, 1, 3]);
            assert_eq!(next_permutation(&mut xs), true);
            assert_eq!(xs, [0, 2, 3, 1]);
        }
    }

    #[test]
    fn many_permutation_step_test() {
        let mut xs = vec![0, 1, 2, 3];
        for _ in 0..23 {
            assert_eq!(next_permutation(&mut xs), true);
        }
        assert_eq!(xs, vec![3, 2, 1, 0]);
        assert_eq!(next_permutation(&mut xs), false);
        assert_eq!(xs, vec![0, 1, 2, 3]);
    }

    #[test]
    fn permutations_iter_test() {
        {
            let mut iter = Permutations::new(vec![0, 1, 2, 3]);
            assert_eq!(iter.next(), Some(vec![0, 1, 2, 3]));
            assert_eq!(iter.next(), Some(vec![0, 1, 3, 2]));
            assert_eq!(iter.next(), Some(vec![0, 2, 1, 3]));
            assert_eq!(iter.next(), Some(vec![0, 2, 3, 1]));
        }
        {
            let iter = Permutations::new(vec![0, 1, 2, 3]);
            assert_eq!(iter.count(), 24);
        }
        {
            let mut iter = Permutations::new(vec![0, 1, 2, 3]);
            let mut v = iter.next().unwrap();
            for w in iter {
                assert!(v < w);
                v = w
            }
        }
    }
}