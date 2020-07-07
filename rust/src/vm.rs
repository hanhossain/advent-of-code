use std::fmt::{Debug, Error, Formatter};
use std::io;

pub struct Intcode<'a> {
    memory: &'a mut [i32],
    instruction_pointer: usize,
}

impl<'a> Intcode<'a> {
    pub fn new(memory: &'a mut [i32]) -> Self {
        Intcode {
            memory,
            instruction_pointer: 0,
        }
    }

    pub fn run(&mut self) {
        loop {
            let instruction = Instruction::read(&mut self.memory, self.instruction_pointer);

            // uncomment when debugging
            // let opcode = self.memory[self.instruction_pointer];
            // println!(
            //     "{} - {} - {:?}",
            //     self.instruction_pointer, opcode, instruction
            // );

            match instruction {
                Instruction::Add(param1, param2, param3) => {
                    param3.set(param1.load() + param2.load());
                    self.instruction_pointer += 4;
                }
                Instruction::Multiply(param1, param2, param3) => {
                    param3.set(param1.load() * param2.load());
                    self.instruction_pointer += 4;
                }
                Instruction::Input(location) => {
                    println!("Input: ");

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();

                    let input: i32 = buffer.trim().parse().unwrap();
                    location.set(input);

                    self.instruction_pointer += 2;
                }
                Instruction::Output(location) => {
                    println!("{}", location.load());
                    self.instruction_pointer += 2;
                }
                Instruction::JumpIfTrue(condition, label) => {
                    let condition = condition.load();
                    if condition != 0 {
                        self.instruction_pointer = label.load() as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::JumpIfFalse(condition, label) => {
                    let condition = condition.load();
                    if condition == 0 {
                        self.instruction_pointer = label.load() as usize;
                    } else {
                        self.instruction_pointer += 3;
                    }
                }
                Instruction::LessThan(left, right, location) => {
                    let left = left.load();
                    let right = right.load();

                    location.set(if left < right { 1 } else { 0 });
                    self.instruction_pointer += 4;
                }
                Instruction::Equals(left, right, location) => {
                    let left = left.load();
                    let right = right.load();

                    location.set(if left == right { 1 } else { 0 });
                    self.instruction_pointer += 4;
                }
                Instruction::Halt => break,
            }
        }
    }
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpIfTrue(Parameter, Parameter),
    JumpIfFalse(Parameter, Parameter),
    LessThan(Parameter, Parameter, Parameter),
    Equals(Parameter, Parameter, Parameter),
    Halt,
}

impl Instruction {
    fn read(memory: &mut [i32], address: usize) -> Self {
        let mem = memory[address];
        let opcode = mem % 100;

        match opcode {
            1 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                let param3 = Parameter::get3(memory, address);

                Instruction::Add(param1, param2, param3)
            }
            2 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                let param3 = Parameter::get3(memory, address);

                Instruction::Multiply(param1, param2, param3)
            }
            3 => {
                let param = Parameter::get1(memory, address);
                Instruction::Input(param)
            }
            4 => {
                let param = Parameter::get1(memory, address);
                Instruction::Output(param)
            }
            5 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                Instruction::JumpIfTrue(param1, param2)
            }
            6 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                Instruction::JumpIfFalse(param1, param2)
            }
            7 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                let param3 = Parameter::get3(memory, address);

                Instruction::LessThan(param1, param2, param3)
            }
            8 => {
                let param1 = Parameter::get1(memory, address);
                let param2 = Parameter::get2(memory, address);
                let param3 = Parameter::get3(memory, address);

                Instruction::Equals(param1, param2, param3)
            }
            99 => Instruction::Halt,
            _ => todo!(),
        }
    }
}

impl Debug for Instruction {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Instruction::Add(x, y, z) => f.write_fmt(format_args!("Add {:?} {:?} {:?}", x, y, z)),
            Instruction::Multiply(x, y, z) => {
                f.write_fmt(format_args!("Multiply {:?} {:?} {:?}", x, y, z))
            }
            Instruction::Input(x) => f.write_fmt(format_args!("Input {:?}", x)),
            Instruction::Output(x) => f.write_fmt(format_args!("Output {:?}", x)),
            Instruction::JumpIfTrue(x, y) => {
                f.write_fmt(format_args!("JumpIfTrue {:?} {:?}", x, y))
            }
            Instruction::JumpIfFalse(x, y) => {
                f.write_fmt(format_args!("JumpIfFalse {:?} {:?}", x, y))
            }
            Instruction::LessThan(x, y, z) => {
                f.write_fmt(format_args!("LessThan {:?} {:?} {:?}", x, y, z))
            }
            Instruction::Equals(x, y, z) => {
                f.write_fmt(format_args!("Equals {:?} {:?} {:?}", x, y, z))
            }
            Instruction::Halt => f.write_str("Halt"),
        }
    }
}

enum Parameter {
    Position(i32, *mut [i32]),
    Immediate(i32),
}

impl Parameter {
    fn get(memory: &mut [i32], mode: i32, param: i32) -> Self {
        match mode {
            0 => Parameter::Position(param, memory),
            1 => Parameter::Immediate(param),
            x => panic!("invalid parameter mode: {}", x),
        }
    }

    fn get1(memory: *mut [i32], address: usize) -> Self {
        let memory: &mut [i32] = unsafe { memory.as_mut().unwrap() };
        let mode = (memory[address] / 100) % 10;
        let param = memory[address + 1];
        Parameter::get(memory, mode, param)
    }

    fn get2(memory: *mut [i32], address: usize) -> Self {
        let memory: &mut [i32] = unsafe { memory.as_mut().unwrap() };
        let mode = (memory[address] / 1_000) % 10;
        let param = memory[address + 2];
        Parameter::get(memory, mode, param)
    }

    fn get3(memory: *mut [i32], address: usize) -> Self {
        let memory: &mut [i32] = unsafe { memory.as_mut().unwrap() };
        let mode = (memory[address] / 10_000) % 10;
        let param = memory[address + 3];
        Parameter::get(memory, mode, param)
    }

    fn load(&self) -> i32 {
        match self {
            Parameter::Position(x, memory) => unsafe { memory.as_mut().unwrap()[*x as usize] },
            Parameter::Immediate(x) => *x,
        }
    }

    fn set(&self, value: i32) {
        match self {
            Parameter::Position(x, memory) => unsafe {
                memory.as_mut().unwrap()[*x as usize] = value
            },
            Parameter::Immediate(_) => panic!("Cannot write to an immediate parameter"),
        }
    }
}

impl Debug for Parameter {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Parameter::Position(x, _) => f.write_fmt(format_args!("{}({})", self.load(), x)),
            Parameter::Immediate(_) => f.write_fmt(format_args!("{}", self.load())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_add() {
        let mut memory = [1, 0, 0, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [2, 0, 0, 0, 99]);
    }

    #[test]
    fn run_add_immediate() {
        let mut memory = [1101, 100, -1, 4, 0];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1101, 100, -1, 4, 99])
    }

    #[test]
    fn run_multiply() {
        let mut memory = [2, 3, 0, 3, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [2, 3, 0, 6, 99]);
    }

    #[test]
    fn run_multiply_2() {
        let mut memory = [2, 4, 4, 5, 99, 0];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [2, 4, 4, 5, 99, 9801]);
    }

    #[test]
    fn run_multiply_immediate() {
        let mut memory = [1102, 2, 4, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [8, 2, 4, 0, 99]);
    }

    #[test]
    fn run_multiply_and_add() {
        let mut memory = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn run_jump_if_true_position_succeeds() {
        let mut memory = [5, 3, 4, 1, 9, 1, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [5, 3, 4, 1, 9, 1, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_true_position_fails() {
        let mut memory = [5, 2, 0, 1, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [2, 2, 0, 1, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_true_immediate_succeeds() {
        let mut memory = [1105, 3, 7, 1101, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1105, 3, 7, 1101, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_true_immediate_fails() {
        let mut memory = [1105, 0, 7, 1101, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [3, 0, 7, 1101, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_false_position_succeeds() {
        let mut memory = [6, 3, 4, 0, 5, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [6, 3, 4, 0, 5, 99]);
    }

    #[test]
    fn run_jump_if_false_position_fails() {
        let mut memory = [6, 0, 1, 1, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1, 0, 1, 1, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_false_immediate_succeeds() {
        let mut memory = [1106, 0, 7, 1101, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1106, 0, 7, 1101, 1, 2, 0, 99]);
    }

    #[test]
    fn run_jump_if_false_immediate_fails() {
        let mut memory = [1106, 1, 7, 1101, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [3, 1, 7, 1101, 1, 2, 0, 99]);
    }

    #[test]
    fn run_less_than_position_succeeds() {
        let mut memory = [7, 2, 0, 3, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [7, 2, 0, 1, 99]);
    }

    #[test]
    fn run_less_than_position_fails() {
        let mut memory = [7, 0, 3, 3, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [7, 0, 3, 0, 99]);
    }

    #[test]
    fn run_less_than_immediate_succeeds() {
        let mut memory = [1107, 1, 2, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1, 1, 2, 0, 99]);
    }

    #[test]
    fn run_less_than_immediate_fails() {
        let mut memory = [1107, 2, 1, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [0, 2, 1, 0, 99]);
    }

    #[test]
    fn run_equals_position_succeeds() {
        let mut memory = [8, 0, 5, 3, 99, 8];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [8, 0, 5, 1, 99, 8]);
    }

    #[test]
    fn run_equals_position_fails() {
        let mut memory = [8, 0, 5, 3, 99, 9];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [8, 0, 5, 0, 99, 9]);
    }

    #[test]
    fn run_equals_immediate_succeeds() {
        let mut memory = [1108, 1, 1, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [1, 1, 1, 0, 99]);
    }

    #[test]
    fn run_equals_immediate_fails() {
        let mut memory = [1108, 1, 4, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [0, 1, 4, 0, 99]);
    }
}
