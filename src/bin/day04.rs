use std::collections::HashMap;

const START: i32 = 245182;
const END: i32 = 790572;

fn digits(n: i32) -> Vec<i32> {
    let mut digits = Vec::<i32>::new();
    assert!(0 <= n && n < 1000000);
    let mut rest = n;
    for _ in 0..6 {
        digits.push(rest % 10);
        rest /= 10;
    }
    digits.reverse();
    digits
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_digits() {
        assert_eq!(crate::digits(0), [0,0,0,0,0,0]);
        assert_eq!(crate::digits(123456), [1,2,3,4,5,6]);
        assert_eq!(crate::digits(17), [0,0,0,0,1,7]);
    }
}

fn is_increasing(digits: &[i32]) -> bool {
    let mut last_digit = 0i32;
    for &d in digits {
        if d < last_digit {
            return false;
        }
        last_digit = d;
    }
    true
}

fn has_double(digits: &[i32]) -> bool {
    let mut last_digit = digits[0];
    for &d in digits[1..].iter() {
        if d == last_digit {
            return true
        }
        last_digit = d
    }
    false
}

fn has_exact_double(digits: &[i32]) -> bool {
    let mut counts: HashMap<i32, i32> = HashMap::new();
    for &d in digits {
        *counts.entry(d).or_insert(0) += 1
    }
    for &v in counts.values() {
        if v == 2 {
            return true;
        }
    }
    false
}

fn main() {
    let mut count = 0i32;
    let mut count_exact = 0i32;
    for n in START..=END {
        let ds = digits(n);
        if is_increasing(&ds) && has_double(&ds) {
            count += 1;
            if has_exact_double(&ds) {
                count_exact += 1;
            }
        }
    }

    println!("{}", count);
    println!("{}", count_exact);
}
