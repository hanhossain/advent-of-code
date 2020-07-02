use crate::vm::Intcode;
use std::fs;

pub fn run_part1() {
    let mut memory = read_file();
    let mut intcode = Intcode::new(&mut memory);
    intcode.run();
}

fn read_file() -> Vec<i32> {
    let filepath = "/Users/hanhossain/Developer/advent-of-code/data/Day05.txt";

    fs::read_to_string(filepath)
        .unwrap()
        .split(",")
        .flat_map(|x| x.parse::<i32>())
        .collect()
}
