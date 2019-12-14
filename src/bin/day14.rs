extern crate regex;
use std::collections::HashMap;
use aoc2019::io::slurp_stdin;

type Spec = (i64, String);
type Reactions = HashMap<String, (Vec<Spec>, i64)>;

fn parse_input(s: &str) -> Reactions {
    let mut ret = Reactions::new();
    let re = regex::Regex::new(r"(\d+) ([A-Z]+)").unwrap();
    for line in s.lines() {
        let mut specs: Vec<Spec> = re.captures_iter(line)
            .map(|m| {
                let amount: i64 = m.get(1).unwrap().as_str().parse().unwrap();
                let name = String::from(m.get(2).unwrap().as_str());
                (amount, name)
            })
            .collect();
        let (result_amount, result_name) = specs.pop().unwrap();
        assert!(ret.get(&result_name).is_none());
        ret.insert(result_name, (specs, result_amount));
    }
    ret
}

fn ceil_div(dividend: i64, divisor: i64) -> i64 {
    (dividend + divisor - 1) / divisor
}

fn ore_needs(reactions: &Reactions) -> i64 {
    let mut needed: HashMap<String, i64> = HashMap::new();
    let mut extra: HashMap<String, i64> = HashMap::new();
    needed.insert(String::from("FUEL"), 1);
    let mut ore_needed = 0i64;

    loop {
        let (name, needed_amount) = match needed.iter().next() {
            None => return ore_needed,
            Some((name, amount)) => (name.clone(), *amount),
        };
        needed.remove(&name);
        let (specs, produced_amount) = reactions.get(&name).expect("Unknown requirement");
        let scale = ceil_div(needed_amount, *produced_amount);
        let surplus = *produced_amount * scale - needed_amount;
        assert!(surplus >= 0);
        assert_eq!(*extra.get(&name).unwrap_or(&0), 0);
        if surplus > 0 {
            extra.insert(name, surplus);
        }
        for (spec_amount, name) in specs {
            let amount = *spec_amount * scale;
            if name == "ORE" {
                ore_needed += amount;
                continue;
            }
            let extra_found = *extra.get(name).unwrap_or(&0);

            if extra_found > amount {
                *extra.get_mut(name).unwrap() -= amount;
            } else {
                extra.remove(name);
                *needed.entry(name.clone()).or_default() += amount - extra_found;
            }
        }
    }
}

fn main() {
    let reactions = parse_input(&slurp_stdin());
    println!("{}", ore_needs(&reactions));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ore1() {
        let ex = r"10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";
        assert_eq!(ore_needs(&parse_input(ex)), 31);
    }

    #[test]
    fn ore2() {
        let ex = r"9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";
        assert_eq!(ore_needs(&parse_input(ex)), 165);
    }

    #[test]
    fn ore3() {
        let ex = r"157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";
        assert_eq!(ore_needs(&parse_input(ex)), 13312);
    }

    #[test]
    fn ore4() {
        let ex = r"2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";
        assert_eq!(ore_needs(&parse_input(ex)), 180697);
    }

    #[test]
    fn ore5() {
        let ex = r"171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";
        assert_eq!(ore_needs(&parse_input(ex)), 2210736);
    }
}