use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::Flatten;

pub fn run_part1() {
    let sum: i32 = read_file()
        .flat_map(|line| line.parse::<i32>())
        .map(|mass| get_fuel_for_mass(mass))
        .sum();

    println!("{}", sum);
}

pub fn run_part2() {
    let sum: i32 = read_file()
        .flat_map(|line| line.parse::<i32>())
        .map(|mass| get_fuel_for_module(mass))
        .sum();

    println!("{}", sum);
}

fn read_file() -> Flatten<Lines<BufReader<File>>> {
    let filepath = "/Users/hanhossain/Developer/advent-of-code/rust/data/year2019/day01.txt";

    let file = File::open(filepath).unwrap();
    BufReader::new(file).lines().flatten()
}

fn get_fuel_for_mass(mass: i32) -> i32 {
    mass / 3 - 2
}

fn get_fuel_for_module(mass: i32) -> i32 {
    let fuel = get_fuel_for_mass(mass);

    if fuel > 0 {
        fuel + get_fuel_for_module(fuel)
    } else {
        0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_fuel_for_mass() {
        assert_eq!(get_fuel_for_mass(12), 2);
        assert_eq!(get_fuel_for_mass(14), 2);
        assert_eq!(get_fuel_for_mass(1969), 654);
        assert_eq!(get_fuel_for_mass(100756), 33583);
    }

    #[test]
    fn test_get_fuel_for_module() {
        assert_eq!(get_fuel_for_module(14), 2);
        assert_eq!(get_fuel_for_module(1969), 966);
        assert_eq!(get_fuel_for_module(100756), 50346);
    }
}
