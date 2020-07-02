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
                Instruction::Add(operand1, operand2, location) => {
                    *location = operand1 + operand2;
                }
                Instruction::Multiply(operand1, operand2, location) => {
                    *location = operand1 * operand2
                }
                Instruction::Halt => break,
            }

            self.instruction_pointer += 4;
        }
    }
}

enum Instruction<'a> {
    Add(i32, i32, &'a mut i32),
    Multiply(i32, i32, &'a mut i32),
    Halt,
}

impl<'a> Instruction<'a> {
    fn read(memory: &'a mut [i32], address: usize) -> Self {
        match memory[address] {
            1 => Instruction::Add(
                memory[memory[address + 1] as usize],
                memory[memory[address + 2] as usize],
                &mut memory[memory[address + 3] as usize],
            ),
            2 => Instruction::Multiply(
                memory[memory[address + 1] as usize],
                memory[memory[address + 2] as usize],
                &mut memory[memory[address + 3] as usize],
            ),
            99 => Instruction::Halt,
            _ => todo!(),
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
    fn run_multiply_and_add() {
        let mut memory = [1, 1, 1, 4, 99, 5, 6, 0, 99];
        let mut intcode = Intcode::new(&mut memory);
        intcode.run();
        assert_eq!(memory, [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
