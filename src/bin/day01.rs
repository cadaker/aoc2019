use std::cmp::max;
use std::io::{self, BufRead};

fn fuel(mass: u64) -> u64 {
    max(mass / 3, 2) - 2
}

fn total_fuel(mass: u64) -> u64 {
    let mut total = 0u64;
    let mut extra_fuel = fuel(mass);
    while extra_fuel > 0 {
        total += extra_fuel;
        extra_fuel = fuel(extra_fuel);
    }
    total
}

#[cfg(test)]
mod tests {
    #[test]
    fn fuel_tests() {
        assert_eq!(crate::fuel(12), 2);
        assert_eq!(crate::fuel(14), 2);
        assert_eq!(crate::fuel(1969), 654);
        assert_eq!(crate::fuel(100756), 33583);
    }

    #[test]
    fn total_fuel_tests() {
        assert_eq!(crate::total_fuel(12), 2);
        assert_eq!(crate::total_fuel(1969), 966);
        assert_eq!(crate::total_fuel(100756), 50346);
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let module_masses: Vec<u64> = handle
        .lines()
        .map(|s| s.unwrap().parse::<u64>().unwrap())
        .collect();

    let module_fuel: u64 = module_masses.iter().map(|&mass| fuel(mass)).sum();
    println!("{}", module_fuel);
    let total: u64 = module_masses.iter().map(|&m| total_fuel(m)).sum();
    println!("{}", total);
}
