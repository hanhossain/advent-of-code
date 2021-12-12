use crate::year2019::vm::Intcode;

const DATA: &str = include_str!("../../data/year2019/day05.txt");

pub fn run() {
    let mut memory = read_file();
    let mut intcode = Intcode::new(&mut memory);
    intcode.run();
}

fn read_file() -> Vec<i32> {
    DATA.split(",")
        .flat_map(|x| x.trim().parse::<i32>())
        .collect()
}
