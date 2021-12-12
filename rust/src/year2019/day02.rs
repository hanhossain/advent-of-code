use crate::year2019::vm::Intcode;

const DATA: &str = include_str!("../../data/year2019/day02.txt");

pub fn run_part1() {
    let result = run(12, 2);
    println!("{}", result);
}

pub fn run_part2() {
    for noun in 0..100 {
        for verb in 0..100 {
            if run(noun, verb) == 19690720 {
                let result = 100 * noun + verb;
                println!("{}", result);
                break;
            }
        }
    }
}

fn run(noun: i32, verb: i32) -> i32 {
    let mut memory = read_file();
    memory[1] = noun;
    memory[2] = verb;
    let mut vm = Intcode::new(&mut memory);
    vm.run();
    memory[0]
}

fn read_file() -> Vec<i32> {
    DATA.split(",").flat_map(|x| x.parse::<i32>()).collect()
}
