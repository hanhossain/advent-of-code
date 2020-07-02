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
                Instruction::Halt => break,
            }
        }
    }
}

enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Multiply(Parameter, Parameter, Parameter),
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
}
