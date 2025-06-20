use std::fs::File;
use std::io::{BufReader, Read};
use crate::instruction::{Instruction, MemAddress, MovMemOperand, MovOperand};
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
            Instruction::MovImm8(register, val) => self.set_register(register, val as u16),
            Instruction::MovImm16(register, val) => self.set_register(register, val),
            Instruction::Mov(dest, src) => {

                match dest {
                    MovOperand::Register(dest_reg) => {
                        match src {
                            MovOperand::Register(src_reg) => self.set_register(dest_reg, self.get_register(src_reg)),
                            MovOperand::Memory(mem_addr) => {
                                let ptr = self.get_ptr_from_mem_address(mem_addr);
                                if !dest_reg.is_8bit() {
                                    let low = self.memory.data[ptr] as u16;
                                    let high = (self.memory.data[ptr + 1] as u16) << 8;
                                    self.set_register(dest_reg, low | high);
                                } else {
                                    self.set_register(dest_reg, self.memory.data[ptr] as u16);
                                }
                            }
                        }
                    }
                    MovOperand::Memory(mem_addr) => {

                        match src {
                            MovOperand::Register(src_reg) => {
                                let ptr = self.get_ptr_from_mem_address(mem_addr);
                                let reg_val = self.get_register(src_reg);

                                self.memory.data[ptr] = reg_val as u8;
                                if !src_reg.is_8bit() {
                                    self.memory.data[ptr + 1] = (reg_val >> 8) as u8;
                                }
                            }
                            MovOperand::Memory(_) => unreachable!()
                        }
                    }
                }
            },
            Instruction::MovAccMem(dest, src) => {
                match (dest, src) {
                    (MovMemOperand::Register(reg), MovMemOperand::MemoryPtr(ptr)) => {
                        if reg.is_8bit() {
                            self.set_register(reg, self.memory.data[ptr as usize] as u16);
                        } else {
                            let val = ((self.memory.data[ptr as usize + 1] as u16) << 8)
                                | self.memory.data[ptr as usize] as u16;
                            self.set_register(reg, val);
                        }
                    }
                    (MovMemOperand::MemoryPtr(ptr), MovMemOperand::Register(reg)) => {
                        if reg.is_8bit() {
                            self.memory.data[ptr as usize] = self.get_register(reg) as u8;
                        } else {
                            let reg_val = self.get_register(reg);
                            let high = reg_val >> 8;
                            let low = reg_val & 0xFF;
                            self.memory.data[ptr as usize] = low as u8;
                            self.memory.data[ptr as usize + 1] = high as u8;
                        }
                    }
                    (_, _) => unreachable!()
                }
            },
        }

        self.set_register(Register::IP, self.get_register(Register::IP) + instruction.get_instr_size());
    }

    pub fn get_ptr_from_mem_address(&self, mem_addr: MemAddress) -> usize {
        let ptr = mem_addr.displacement + if let Some(base_reg) = mem_addr.base {
            self.get_register(base_reg)
        } else {
            0
        } + if let Some(index_reg) = mem_addr.index {
            self.get_register(index_reg)
        } else {
            0
        };

        ptr as usize
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
                    _ => unreachable!()
                };
                (self.registers[base_reg as usize] & 0xFF00) >> 8
            },

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => unreachable!()
                };
                self.registers[base_reg as usize] & 0x00FF
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
                    _ => unreachable!()
                };
                self.registers[base_reg as usize] = (self.get_register(base_reg) & 0x00FF) | ((value & 0x00FF) << 8);
            }

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => unreachable!()
                };
                self.registers[base_reg as usize] = (self.get_register(base_reg) & 0xFF00) | (value & 0x00FF);
            }
        }
    }

    pub fn memory(&self) -> &LinearMemory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut LinearMemory {
        &mut self.memory
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