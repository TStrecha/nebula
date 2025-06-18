use std::fs::File;
use std::io::{BufReader, Read};
use crate::instruction::Instruction;
use crate::memory::LinearMemory;
use crate::register::Register;

pub struct Machine {
    memory: LinearMemory,
    registers: [u16; 14],
}

impl Machine {
    pub fn load_program(&mut self, program: BufReader<File>) {
        assert!(program.get_ref().metadata().unwrap().len() <= self.memory.data.len() as u64, "Program cannot be larger than memory");

        for (i, byte) in program.bytes().enumerate() {
            self.memory.data[i] = byte.unwrap();
        }
    }

    pub fn load_program_bytes(&mut self, program: &[u8]) {
        assert!(program.len() <= program.len(), "Program cannot be larger than memory");

        self.memory.data[..program.len()].copy_from_slice(program);
    }

    pub fn step(&mut self) {
        let ip = self.get_register(Register::IP) as usize;
        let memory_slice = &self.memory.data[ip + 1 .. ip + 11];

        let opcode_byte = self.memory.data[ip];
        let instruction = Instruction::from_bytes(opcode_byte, memory_slice).unwrap();
        println!("Running instruction: {:?}", instruction);

        match instruction {
            Instruction::Noop => {},
            Instruction::Mov8(register, val) => self.set_register(register, val as u16),
            Instruction::Mov16(register, val) => self.set_register(register, val),
        }

        self.set_register(Register::IP, self.get_register(Register::IP) + instruction.get_instr_size());
    }

    pub fn memory(&self) -> &LinearMemory {
        &self.memory
    }

    pub fn get_register(&self, register: Register) -> u16 {
        use Register::*;

        match register {
            AX | BX | CX | DX | SI | DI | SP | BP | CS | DS | SS | ES | IP | F => {
                self.registers[register as usize]
            },

            AH | BH | CH | DH => {
                let base_reg = match register {
                    AH => AX,
                    BH => BX,
                    CH => CX,
                    DH => DX,
                    _ => panic!("Not possible")
                };
                (self.get_register(base_reg) & 0xFF00) >> 8
            },

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => panic!("Not possible")
                };
                self.get_register(base_reg) & 0x00FF
            }
        }
    }

    pub fn set_register(&mut self, register: Register, value: u16) {
        use Register::*;

        match register {
            AX | BX | CX | DX | SI | DI | SP | BP | CS | DS | SS | ES | IP | F => {
                self.registers[register as usize] = value
            },

            AH | BH | CH | DH => {
                let base_reg = match register {
                    AH => AX,
                    BH => BX,
                    CH => CX,
                    DH => DX,
                    _ => panic!("Not possible")
                };
                self.set_register(base_reg, (self.get_register(base_reg) & 0x00FF) | ((value & 0x00FF) << 8));
            }

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => panic!("Not possible")
                };
                self.set_register(base_reg, (self.get_register(base_reg) & 0xFF00) | (value & 0x00FF));
            }
        }
    }
}

impl Default for Machine {
    fn default() -> Self {
        let mut machine = Self {
            memory: LinearMemory::default(),
            registers: [0; 14]
        };

        machine.set_register(Register::SP, 1024);

        machine
    }
}