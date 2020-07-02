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
            match Instruction::read(&mut self.memory, self.instruction_pointer) {
                Instruction::Add(param1, param2, param3) => {
                    if let Parameter::Position(location) = param3 {
                        unsafe {
                            *location = param1.load() + param2.load();
                        }
                        self.instruction_pointer += 4;
                    } else {
                        panic!("Cannot write to an immediate parameter");
                    }
                }
                Instruction::Multiply(param1, param2, param3) => {
                    if let Parameter::Position(location) = param3 {
                        unsafe {
                            *location = param1.load() * param2.load();
                        }
                        self.instruction_pointer += 4;
                    } else {
                        panic!("Cannot write to an immediate parameter");
                    }
                }
                Instruction::Input(location) => {
                    println!("Input: ");

                    let mut buffer = String::new();
                    io::stdin().read_line(&mut buffer).unwrap();

                    let input: i32 = buffer.trim().parse().unwrap();
                    if let Parameter::Position(location) = location {
                        unsafe {
                            *location = input;
                        }
                    } else {
                        panic!("Cannot write to an immediate parameter");
                    }

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
            99 => Instruction::Halt,
            _ => todo!(),
        }
    }
}

enum Parameter {
    Position(*mut i32),
    Immediate(i32),
}

impl Parameter {
    fn get1(memory: &mut [i32], address: usize) -> Self {
        let mode = (memory[address] / 100) % 10;
        let param = memory[address + 1];

        if mode == 0 {
            Parameter::Position(&mut memory[param as usize] as *mut i32)
        } else {
            Parameter::Immediate(param)
        }
    }

    fn get2(memory: &mut [i32], address: usize) -> Self {
        let mode = (memory[address] / 1_000) % 10;
        let param = memory[address + 2];

        if mode == 0 {
            Parameter::Position(&mut memory[param as usize] as *mut i32)
        } else {
            Parameter::Immediate(param)
        }
    }

    fn get3(memory: &mut [i32], address: usize) -> Self {
        let mode = (memory[address] / 10_000) % 10;
        let param = memory[address + 3];

        if mode == 0 {
            Parameter::Position(&mut memory[param as usize] as *mut i32)
        } else {
            Parameter::Immediate(param)
        }
    }

    fn load(&self) -> i32 {
        match self {
            Parameter::Position(x) => unsafe { **x },
            Parameter::Immediate(x) => *x,
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
}
