use crate::instruction::{Instruction, MovMemOperand};
use crate::Machine;
use crate::modrm::Operand;
use crate::register::{Flag, Register};

impl Machine {

    pub fn run_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Noop => {},
            Instruction::MovImm8(register, val) => self.set_register(register, val as u16),
            Instruction::MovImm16(register, val) => self.set_register(register, val),
            Instruction::Mov(dest, src) => {
                self.apply_binary_op(dest, src, |_, b| b);
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
            Instruction::Add(dest, src, ..) => {
                let (.., result) = self.apply_binary_op(dest, src, |a, b| a.wrapping_add(b));
                self.update_zero_flag(result);
            }
            Instruction::AddAcc8(val) => {
                let result = self.get_register(Register::AL).wrapping_add(val as u16);
                self.set_register(Register::AL, result);
                self.update_zero_flag(result);
            }
            Instruction::AddAcc16(val) => {
                let result = self.get_register(Register::AX).wrapping_add(val);
                self.set_register(Register::AX, result);
                self.update_zero_flag(result);
            }
            Instruction::Sub(dest, src, ..) => {
                let (.., result) = self.apply_binary_op(dest, src, |a, b| a.wrapping_sub(b));
                self.update_zero_flag(result);
            }
            Instruction::SubAcc8(val) => {
                let result = self.get_register(Register::AL).wrapping_sub(val as u16);
                self.set_register(Register::AL, result);
                self.update_zero_flag(result);
            }
            Instruction::SubAcc16(val) => {
                let result = self.get_register(Register::AX).wrapping_sub(val);
                self.set_register(Register::AX, result);
                self.update_zero_flag(result);
            }
            Instruction::Inc(reg) => {
                let result = self.get_register(reg).wrapping_add(1);
                self.set_register(reg, result);
                self.update_zero_flag(result);
            }
            Instruction::Dec(reg) => {
                let result = self.get_register(reg).wrapping_sub(1);
                self.set_register(reg, result);
                self.update_zero_flag(result);
            }
            Instruction::And(dest, src) => {
                let (.., result) = self.apply_binary_op(dest, src, |a, b| a & b);
                self.update_zero_flag(result);
            }
            Instruction::AndAcc8(val) => {
                let result = self.get_register(Register::AL) & val as u16;
                self.set_register(Register::AL, result);
                self.update_zero_flag(result);
            }
            Instruction::AndAcc16(val) => {
                let result = self.get_register(Register::AX) & val;
                self.set_register(Register::AX, result);
                self.update_zero_flag(result);
            }
            Instruction::Or(dest, src) => {
                let (.., result) = self.apply_binary_op(dest, src, |a, b| a | b);
                self.update_zero_flag(result);
            }
            Instruction::OrAcc8(val) => {
                let result = self.get_register(Register::AL) | val as u16;
                self.set_register(Register::AL, result);
                self.update_zero_flag(result);
            }
            Instruction::OrAcc16(val) => {
                let result = self.get_register(Register::AX) | val;
                self.set_register(Register::AX, result);
                self.update_zero_flag(result);
            }
            Instruction::Mul8(mlt_src) => {
                let multiplier = match mlt_src {
                    Operand::Register(reg) => {
                        self.get_register(reg)
                    }
                    Operand::Memory(addr) => {
                        self.memory.read_byte(self.get_ptr_from_mem_address(addr)) as u16
                    }
                };

                let al = self.get_register(Register::AL) as u8;
                let product = (al as u16) * multiplier;

                self.set_register(Register::AL, product & 0x00FF);
                self.set_register(Register::AH, product >> 8);
            }
            Instruction::Mul16(mlt_src) => {
                let multiplier = match mlt_src {
                    Operand::Register(reg) => {
                        self.get_register(reg)
                    }
                    Operand::Memory(addr) => {
                        self.memory.read_word(self.get_ptr_from_mem_address(addr))
                    }
                };

                let ax = self.get_register(Register::AX);
                let product = (ax as u32) * (multiplier as u32);

                self.set_register(Register::AX, (product & 0xFFFF) as u16);
                self.set_register(Register::DX, (product >> 16) as u16);
            }
            Instruction::Div8(div_src) => {
                let dividend = self.get_register(Register::AX);
                let divisor = match div_src {
                    Operand::Register(reg) => {
                        self.get_register(reg)
                    }
                    Operand::Memory(addr) => {
                        self.memory.read_word(self.get_ptr_from_mem_address(addr))
                    }
                };

                let quotient = dividend / divisor;
                let remainder = dividend % divisor;

                self.set_register(Register::AL, quotient);
                self.set_register(Register::AH, remainder);
            }
            Instruction::Div16(div_src) => {
                let dividend = (self.get_register(Register::DX) as u32) << 16 | self.get_register(Register::AX) as u32;
                let divisor = match div_src {
                    Operand::Register(reg) => {
                        self.get_register(reg)
                    }
                    Operand::Memory(addr) => {
                        self.memory.read_word(self.get_ptr_from_mem_address(addr))
                    }
                };

                let quotient = dividend / divisor as u32;
                let remainder = dividend % divisor as u32;

                self.set_register(Register::AX, quotient as u16);
                self.set_register(Register::DX, remainder as u16);
            }
            Instruction::JmpNear(offset) => {
                let ip = self.get_register(Register::IP) as i16;
                self.set_register(Register::IP, ip.wrapping_add(offset) as u16);
            }
            Instruction::JmpFar(segment, offset) => {
                self.set_register(Register::CS, segment);
                self.set_register(Register::IP, offset);
            }
            Instruction::Jz(offset) => {
                if self.get_flag(Flag::ZERO) {
                    let ip = self.get_register(Register::IP) as i16;
                    self.set_register(Register::IP, ip.wrapping_add(offset as i16) as u16);
                }
            }
            Instruction::Jnz(offset) => {
                if !self.get_flag(Flag::ZERO) {
                    let ip = self.get_register(Register::IP) as i16;
                    self.set_register(Register::IP, ip.wrapping_add(offset as i16) as u16);
                }
            }
            Instruction::JmpShort(offset) => {
                let ip = self.get_register(Register::IP) as i16;
                self.set_register(Register::IP, ip.wrapping_add(offset as i16) as u16);
            }
        }
    }

    fn apply_binary_op<F>(&mut self, dest: Operand, src: Operand, op: F) -> (u16, u16, u16)
    where
        F: Fn(u16, u16) -> u16,
    {
        match (dest, src) {
            (Operand::Register(dest_reg), Operand::Register(src_reg)) => {
                let lhs = self.get_register(dest_reg);
                let rhs = self.get_register(src_reg);
                let result = op(lhs, rhs);
                self.set_register(dest_reg, result);

                (lhs, rhs, result)
            }
            (Operand::Register(dest_reg), Operand::Memory(mem_addr)) => {
                let ptr = self.get_ptr_from_mem_address(mem_addr);
                let lhs = self.get_register(dest_reg);
                let rhs = if dest_reg.is_8bit() {
                    self.memory.read_byte(ptr) as u16
                } else {
                    self.memory.read_word(ptr)
                };
                let result = op(lhs, rhs);
                self.set_register(dest_reg, op(lhs, rhs));

                (lhs, rhs, result)
            }
            (Operand::Memory(mem_addr), Operand::Register(src_reg)) => {
                let ptr = self.get_ptr_from_mem_address(mem_addr);
                let lhs = if src_reg.is_8bit() {
                    self.memory.read_byte(ptr) as u16
                } else {
                    self.memory.read_word(ptr)
                };
                let rhs = self.get_register(src_reg);
                let result = op(lhs, rhs);
                if src_reg.is_8bit() {
                    self.memory.write_byte(ptr, result as u8);
                } else {
                    self.memory.write_word(ptr, result);
                }

                (lhs, rhs, result)
            }
            (Operand::Memory(_), Operand::Memory(_)) => unreachable!(),
        }
    }

}