use std::fs::File;
use std::io::{BufReader, Read};
use crate::instruction::{Instruction, MovMemOperand};
use crate::memory::LinearMemory;
use crate::modrm::{MemAddress, Operand};
use crate::register::Register;

pub struct Machine {
    memory: LinearMemory,
    registers: [u16; 14],
}

impl Machine {
    pub fn load_program(&mut self, program: BufReader<File>) {
        assert!(program.get_ref().metadata().unwrap().len() <= self.memory.data.len() as u64, "Program cannot be larger than memory");

        for (i, byte) in program.bytes().enumerate() {
            self.memory.write_byte(i, byte.unwrap());
        }
    }

    pub fn load_program_bytes(&mut self, program: &[u8]) {
        assert!(program.len() <= program.len(), "Program cannot be larger than memory");

        self.memory.data[..program.len()].copy_from_slice(program);
    }

    pub fn step(&mut self) {
        let ip = self.get_register(Register::IP) as usize;

        let opcode_byte = self.memory.read_byte(ip);
        let instruction = Instruction::from_bytes(opcode_byte, &self.memory.data[ip + 1..]).unwrap();
        println!("Running instruction: {:?}", instruction);

        self.run_instruction(instruction);

        self.set_register(Register::IP, self.get_register(Register::IP) + instruction.get_instr_size());
    }

    pub fn run_instruction(&mut self, instruction: Instruction) {

        match instruction {
            Instruction::Noop => {},
            Instruction::MovImm8(register, val) => self.set_register(register, val as u16),
            Instruction::MovImm16(register, val) => self.set_register(register, val),
            Instruction::Mov(dest, src) => {

                match(dest, src) {
                    (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
                        self.set_register(dest_reg, self.get_register(src_reg))
                    },
                    (Operand::Register(dest_reg), Operand::Memory(mem_addr)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        if dest_reg.is_8bit() {
                            self.set_register(dest_reg, self.memory.read_byte(ptr) as u16);
                        } else {
                            self.set_register(dest_reg, self.memory.read_word(ptr));
                        }
                    },
                    (Operand::Memory(mem_addr), Operand::Register(src_reg)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        let reg_val = self.get_register(src_reg);

                        if src_reg.is_8bit() {
                            self.memory.write_byte(ptr, reg_val as u8);
                        } else {
                            self.memory.write_word(ptr, reg_val);
                        }
                    }
                    (Operand::Memory(_), Operand::Memory(_)) => unreachable!()
                }
            },
            Instruction::MovAccMem(dest, src) => {
                match (dest, src) {
                    (MovMemOperand::Register(reg), MovMemOperand::MemoryPtr(ptr)) => {
                        if reg.is_8bit() {
                            self.set_register(reg, self.memory.read_byte(ptr as usize) as u16);
                        } else {
                            self.set_register(reg, self.memory.read_word(ptr as usize));
                        }
                    }
                    (MovMemOperand::MemoryPtr(ptr), MovMemOperand::Register(reg)) => {
                        if reg.is_8bit() {
                            self.memory.write_byte(ptr as usize, self.get_register(reg) as u8);
                        } else {
                            self.memory.write_word(ptr as usize, self.get_register(reg));
                        }
                    }
                    (_, _) => unreachable!()
                }
            },
            Instruction::Push(reg) => {
                self.set_register(Register::SP, self.get_register(Register::SP) - 2);
                self.memory.write_word(self.get_register(Register::SP) as usize, self.get_register(reg));
            },
            Instruction::Pop(reg) => {
                self.set_register(reg, self.memory.read_word(self.get_register(Register::SP) as usize));
                self.set_register(Register::SP, self.get_register(Register::SP) + 2);
            },
            Instruction::Add(dest, src) => {
                match(dest, src) {
                    (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
                        self.set_register(dest_reg, self.get_register(dest_reg) + self.get_register(src_reg))
                    },
                    (Operand::Register(dest_reg), Operand::Memory(mem_addr)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        if dest_reg.is_8bit() {
                            self.set_register(dest_reg, self.get_register(dest_reg) + self.memory.read_byte(ptr) as u16);
                        } else {
                            self.set_register(dest_reg, self.get_register(dest_reg) + self.memory.read_word(ptr));
                        }
                    },
                    (Operand::Memory(mem_addr), Operand::Register(src_reg)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        let reg_val = self.get_register(src_reg);

                        if src_reg.is_8bit() {
                            self.memory.write_byte(ptr, self.memory.read_byte(ptr) + reg_val as u8);
                        } else {
                            self.memory.write_word(ptr, self.memory.read_word(ptr) + reg_val);
                        }
                    }
                    (Operand::Memory(_), Operand::Memory(_)) => unreachable!()
                }
            }
            Instruction::AddAcc8(val) => {
                self.set_register(Register::AL, self.get_register(Register::AL) + val as u16);
            }
            Instruction::AddAcc16(val) => {
                self.set_register(Register::AX, self.get_register(Register::AX) + val);
            }
            Instruction::Sub(dest, src) => {
                match(dest, src) {
                    (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
                        self.set_register(dest_reg, self.get_register(dest_reg) - self.get_register(src_reg))
                    },
                    (Operand::Register(dest_reg), Operand::Memory(mem_addr)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        if dest_reg.is_8bit() {
                            self.set_register(dest_reg, self.get_register(dest_reg) - self.memory.read_byte(ptr) as u16);
                        } else {
                            self.set_register(dest_reg, self.get_register(dest_reg) - self.memory.read_word(ptr));
                        }
                    },
                    (Operand::Memory(mem_addr), Operand::Register(src_reg)) => {
                        let ptr = self.get_ptr_from_mem_address(mem_addr);
                        let src_reg_val = self.get_register(src_reg);

                        if src_reg.is_8bit() {
                            self.memory.write_byte(ptr, self.memory.read_byte(ptr) - src_reg_val as u8);
                        } else {
                            self.memory.write_word(ptr, self.memory.read_word(ptr) - src_reg_val);
                        }
                    }
                    (Operand::Memory(_), Operand::Memory(_)) => unreachable!()
                }
            }
            Instruction::SubAcc8(val) => {
                self.set_register(Register::AL, self.get_register(Register::AL) - val as u16);
            }
            Instruction::SubAcc16(val) => {
                self.set_register(Register::AX, self.get_register(Register::AX) - val);
            }
        }
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