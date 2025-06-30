use crate::instruction::Instruction;
use crate::memory::LinearMemory;
use crate::modrm::MemAddress;
use crate::register::{Flag, Register};
use std::fs::File;
use std::io::{BufReader, Read};

pub struct Machine {
    pub(super) memory: LinearMemory,
    registers: [u16; 14],
}

impl Machine {
    pub fn load_program(&mut self, program: BufReader<File>) {
        assert!(
            program.get_ref().metadata().unwrap().len() <= self.memory.data.len() as u64,
            "Program cannot be larger than memory"
        );

        for (i, byte) in program.bytes().enumerate() {
            self.memory.write_byte(i, byte.unwrap());
        }
    }

    pub fn load_program_bytes(&mut self, program: &[u8]) {
        assert!(
            program.len() <= program.len(),
            "Program cannot be larger than memory"
        );

        self.memory.data[..program.len()].copy_from_slice(program);
    }

    pub fn step(&mut self) {
        let ip = self.get_register(Register::IP) as usize;

        print!("Running instruction at 0x{:x}", ip);
        let opcode_byte = self.memory.read_byte(ip);
        let instruction =
            Instruction::from_bytes(opcode_byte, &self.memory.data[ip + 1..]).unwrap();
        println!(": {:?}", instruction);

        self.run_instruction(instruction);

        self.set_register(
            Register::IP,
            self.get_register(Register::IP) + instruction.get_instr_size(),
        );
    }

    pub fn get_ptr_from_mem_address(&self, mem_addr: MemAddress) -> usize {
        let ptr = mem_addr.displacement
            + if let Some(base_reg) = mem_addr.base {
                self.get_register(base_reg)
            } else {
                0
            }
            + if let Some(index_reg) = mem_addr.index {
                self.get_register(index_reg)
            } else {
                0
            };

        ptr as usize
    }

    pub fn update_zero_flag(&mut self, value: u16) {
        self.set_flag(Flag::ZERO, value == 0);
    }

    pub fn get_flag(&self, flag: Flag) -> bool {
        self.get_register(Register::F) & flag as u16 != 0
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        let mut flag_reg = self.get_register(Register::F);
        let mask = flag as u16;
        if value {
            flag_reg |= mask;
        } else {
            flag_reg &= !mask;
        }
        self.set_register(Register::F, flag_reg);
    }

    pub fn get_register(&self, register: Register) -> u16 {
        use Register::*;

        match register {
            AX | BX | CX | DX | SI | DI | SP | BP | CS | DS | SS | ES | IP | F => {
                self.registers[register as usize]
            }

            AH | BH | CH | DH => {
                let base_reg = match register {
                    AH => AX,
                    BH => BX,
                    CH => CX,
                    DH => DX,
                    _ => unreachable!(),
                };
                (self.registers[base_reg as usize] & 0xFF00) >> 8
            }

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => unreachable!(),
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
            }

            AH | BH | CH | DH => {
                let base_reg = match register {
                    AH => AX,
                    BH => BX,
                    CH => CX,
                    DH => DX,
                    _ => unreachable!(),
                };
                self.registers[base_reg as usize] =
                    (self.get_register(base_reg) & 0x00FF) | ((value & 0x00FF) << 8);
            }

            AL | BL | CL | DL => {
                let base_reg = match register {
                    AL => AX,
                    BL => BX,
                    CL => CX,
                    DL => DX,
                    _ => unreachable!(),
                };
                self.registers[base_reg as usize] =
                    (self.get_register(base_reg) & 0xFF00) | (value & 0x00FF);
            }
        }
    }

    pub fn memory(&self) -> &LinearMemory {
        &self.memory
    }

    pub fn memory_mut(&mut self) -> &mut LinearMemory {
        &mut self.memory
    }

    pub fn dump_self(&self) {
        println!(
            "AX: {} | CX: {} | DX: {} | BX: {} | SP: {} | \
        BP: {} | SI: {} | DI: {} | CS: {} | DS: {} | SS: {} | \
        ES: {} | IP: {} | FLAGS: {:b}",
            self.get_register(Register::AX),
            self.get_register(Register::CX),
            self.get_register(Register::DX),
            self.get_register(Register::BX),
            self.get_register(Register::SP),
            self.get_register(Register::BP),
            self.get_register(Register::SI),
            self.get_register(Register::DI),
            self.get_register(Register::CS),
            self.get_register(Register::DS),
            self.get_register(Register::SS),
            self.get_register(Register::ES),
            self.get_register(Register::IP),
            self.get_register(Register::F)
        );
    }
}

impl Default for Machine {
    fn default() -> Self {
        let mut machine = Self {
            memory: LinearMemory::default(),
            registers: [0; 14],
        };

        machine.set_register(Register::SP, 1024);

        machine
    }
}

