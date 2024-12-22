//! NOT WORKING YET, AT LEAST NOT FAST ENOUGH TO ACTUALLY FIND THE SOLUTION

use core::fmt;
use regex::Regex;
use std::io::{self, stdin, Read};

fn main() {
    let mut puzzle_input = String::new();
    stdin().read_to_string(&mut puzzle_input).unwrap();

    let cpu = Cpu::new(&puzzle_input, Vec::new());

    // Search for the magic register value that causes the program to output its own code
    let magic_register_value = (0..)
        .find(|magic_register_value| {
            let mut cpu = cpu.clone();
            cpu.registers[0] = *magic_register_value;
            cpu.run();
            let program_output = String::from_utf8_lossy(&cpu.output);
            let program_code = cpu
                .program
                .into_iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .join(",");
            if magic_register_value % 1000000 == 0 {
                dbg!(magic_register_value);
                dbg!(&program_code.len());
                dbg!(&program_output.len());
            }
            program_output == program_code
        })
        .expect("The range is infinite, so the program will eventually find a magic register value or will run forever");
    dbg!(magic_register_value);
}

#[derive(Clone)]
struct Cpu<W: io::Write> {
    /// Registers A - C.
    registers: [i64; 3],
    /// The program is a list of instructions made up of opcode and operand.
    program: Vec<u8>,
    /// Current position in the program.
    instruction_pointer: usize,
    /// Output buffer for [`out`](Self::out) instructions.
    output: W,
    /// Indicates whether any output was written to [`Self::output`].
    output_used: bool,
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    opcode: u8,
    operand: u8,
}

impl From<&[u8]> for Instruction {
    /// # Panics
    /// Panics if `value.len() != 2`.
    fn from(value: &[u8]) -> Self {
        assert!(value.len() == 2);
        Self {
            opcode: value[0],
            operand: value[1],
        }
    }
}

impl<W: io::Write> Cpu<W> {
    fn new(init_state: &str, output: W) -> Cpu<W> {
        let (register_init, program) = init_state.split_once("\n\n").unwrap();

        let mut cpu = Cpu {
            registers: Default::default(),
            program: Default::default(),
            instruction_pointer: Default::default(),
            output,
            output_used: false,
        };
        cpu.init_registers(register_init);
        cpu.init_program(program);
        cpu
    }

    /// Runs the [instructions](Self::instructions) until the program attempts to read an opcode past the end of the program.
    fn run(&mut self) {
        // Since instructions are two words long, check if the full instruction is inside bounds
        while self.instruction_pointer + 1 < self.program.len() {
            self.instruction();
        }
    }

    /// Performs the instruction at [`Self::instruction_pointer`].
    fn instruction(&mut self) {
        let instruction = Instruction::from(
            &self.program[self.instruction_pointer..=self.instruction_pointer + 1],
        );

        let old_instruction_pointer = self.instruction_pointer;
        match instruction.opcode {
            0 => self.adv(instruction.operand as i64),
            1 => self.bxl(instruction.operand as i64),
            2 => self.bst(instruction.operand as i64),
            3 => self.jnz(instruction.operand as i64),
            4 => self.bxc(instruction.operand as i64),
            5 => self.out(instruction.operand as i64),
            6 => self.bdv(instruction.operand as i64),
            7 => self.cdv(instruction.operand as i64),
            _ => panic!("Unknown opcode: {}", instruction.opcode),
        }
        let jump_occurred = self.instruction_pointer != old_instruction_pointer;
        if !jump_occurred {
            self.instruction_pointer += 2;
        }
    }

    /// Division
    fn adv(&mut self, literal_operand: i64) {
        let numerator = self.registers[0];
        let denominator = 2_i64.pow(self.combo_operand(literal_operand) as u32);
        self.registers[0] = numerator / denominator;
    }

    /// register[B] = register[B] XOR literal_operand
    fn bxl(&mut self, literal_operand: i64) {
        self.registers[1] ^= literal_operand;
    }

    /// register[B] = combo_operand % 8
    fn bst(&mut self, literal_operand: i64) {
        self.registers[1] = self.combo_operand(literal_operand) % 8;
    }

    /// Jump if not zero
    fn jnz(&mut self, literal_operand: i64) {
        if self.registers[0] == 0 {
            return;
        }
        self.instruction_pointer = literal_operand as usize;
    }

    /// register[B] = register[B] XOR register[C]
    fn bxc(&mut self, _operand: i64) {
        self.registers[1] ^= self.registers[2];
    }

    /// print(combo_operand % 8)
    fn out(&mut self, literal_operand: i64) {
        let value = self.combo_operand(literal_operand) % 8;
        if self.output_used {
            // Prepend output with a comma if output was already used before
            write!(self.output, ",").unwrap();
        }
        write!(self.output, "{}", value).expect("Writing value of out instruction to output");
        self.output_used = true;
    }

    /// Division into B register.
    fn bdv(&mut self, literal_operand: i64) {
        let numerator = self.registers[0];
        let denominator = 2_i64.pow(self.combo_operand(literal_operand) as u32);
        self.registers[1] = numerator / denominator;
    }

    /// Division into C register.
    fn cdv(&mut self, literal_operand: i64) {
        let numerator = self.registers[0];
        let denominator = 2_i64.pow(self.combo_operand(literal_operand) as u32);
        self.registers[2] = numerator / denominator;
    }

    /// Returns the value of the combo operand
    fn combo_operand(&self, literal_operand: i64) -> i64 {
        match literal_operand {
            literal_value @ 0..=3 => literal_value,
            register_number @ 4..=6 => {
                let register_number = register_number - 4;
                assert!((0..=3).contains(&register_number));
                self.registers[register_number as usize]
            }
            7 => panic!("Combo operand 7 is reserved and will not appear in valid programs"),
            _ => panic!("Combo operand {} out of range 0-7", literal_operand),
        }
    }

    fn init_registers(&mut self, register_init: &str) {
        let register_init_regex = Regex::new(r"Register (\w): (\d+)").unwrap();

        let register_init = register_init.lines().map(|line| {
            let register_init_capture = register_init_regex.captures(line).unwrap();
            let register_name = register_init_capture.get(1).unwrap().as_str();
            let register_number = match register_name {
                "A" => 0,
                "B" => 1,
                "C" => 2,
                _ => panic!("Unknown register name: {}", register_name),
            };
            let register_value = register_init_capture
                .get(2)
                .unwrap()
                .as_str()
                .parse()
                .unwrap();
            (register_number, register_value)
        });

        for (register_number, register_value) in register_init {
            self.registers[register_number] = register_value;
        }
    }

    fn init_program(&mut self, program: &str) {
        let program = program.strip_prefix("Program: ").unwrap();
        self.program = program
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect::<Vec<_>>();
    }
}

impl<W: io::Write> fmt::Debug for Cpu<W> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Cpu")
            .field("registers", &self.registers)
            .field("instructions", &self.program)
            .field("instruction_pointer", &self.instruction_pointer)
            .finish()
    }
}

/// These tests are based on the instruction operation examples in the task description.
#[cfg(test)]
mod tests {
    use crate::Cpu;

    #[test]
    fn prog1() {
        let mut cpu = Cpu {
            registers: [0, 0, 9],
            program: [2, 6].to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
            output_used: false,
        };
        cpu.run();
        assert_eq!(cpu.registers[1], 1);
    }

    #[test]
    fn prog2() {
        let mut cpu = Cpu {
            registers: [10, 0, 0],
            program: [5, 0, 5, 1, 5, 4].to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
            output_used: false,
        };
        cpu.run();
        assert_eq!(String::from_utf8_lossy(&cpu.output), "0,1,2");
    }

    #[test]
    fn prog3() {
        let mut cpu = Cpu {
            registers: [2024, 0, 0],
            program: [0, 1, 5, 4, 3, 0].to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
            output_used: false,
        };
        cpu.run();
        assert_eq!(
            String::from_utf8_lossy(&cpu.output),
            "4,2,5,6,7,7,7,7,3,1,0"
        );
    }

    #[test]
    fn prog4() {
        let mut cpu = Cpu {
            registers: [0, 29, 0],
            program: [1, 7].to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
            output_used: false,
        };
        cpu.run();
        assert_eq!(cpu.registers[1], 26);
    }

    #[test]
    fn prog5() {
        let mut cpu = Cpu {
            registers: [0, 2024, 43690],
            program: [4, 0].to_vec(),
            instruction_pointer: 0,
            output: Vec::new(),
            output_used: false,
        };
        cpu.run();
        assert_eq!(cpu.registers[1], 44354);
    }
}
