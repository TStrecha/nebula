use crate::modrm::{decode_operands_from_mod_rm_opcode, Operand};
use crate::register::Register;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Opcode {
    NOOP = 0x90,
    PUSH,
    POP,
    MOV_IMM = 0xB0, // B0 - BF, MOV reg, imm8/16 (immediate to register)
    MOV_REG_MEM = 0x88, // 88 - 8B, MOV r/m, r || MOV r, r/m
    MOV_ACC_MEM = 0xA0, // A0 - A3, MOV AL/AX <-> [imm16]
    ADD = 0x00,
    ADD_ACC_8 = 0x04,
    ADD_ACC_16 = 0x05,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MovMemOperand {
    Register(Register),
    MemoryPtr(u16),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Instruction {
    Noop,
    MovImm8(Register, u8),
    MovImm16(Register, u16),
    // MOV between register and memory or register <-> register
    //  DEST      , SRC
    Mov(Operand, Operand),
    // MOV AL/AX <-> [imm16]
    //  DEST      , SRC
    MovAccMem(MovMemOperand, MovMemOperand),
    Push(Register),
    Pop(Register),
    Add(Operand, Operand),
    AddAcc8(u8),
    AddAcc16(u16),
}

impl Instruction {
    pub fn from_bytes(opcode_byte: u8, memory_slice: &[u8]) -> Result<Self, String> {
        let opcode = Opcode::try_from(opcode_byte)?;

        match opcode {
            Opcode::NOOP => Ok(Self::Noop),
            Opcode::MOV_IMM => {
                let bits_8 = (opcode_byte & 0b00001000) == 0;
                let reg_bits = opcode_byte & 0b00000111;
                let reg = Register::from_register_code(reg_bits, bits_8)?;

                let instr = if bits_8 {
                    let val = memory_slice[0];
                    Self::MovImm8(reg, val)
                } else {
                    let upper = memory_slice[1] as u16;
                    let lower = memory_slice[0] as u16;
                    Self::MovImm16(reg, (upper << 8) | lower)
                };

                Ok(instr)
            },
            Opcode::MOV_REG_MEM => {
                let operands = decode_operands_from_mod_rm_opcode(opcode_byte, memory_slice)?;
                Ok(Self::Mov(operands.0, operands.1))
            }
            Opcode::MOV_ACC_MEM => {
                let is_reg_target = opcode_byte & 0b00000010 == 0;
                let is_8_bit = opcode_byte & 0b00000001 == 0;

                let register = if is_8_bit {
                    Register::AL
                } else {
                    Register::AX
                };

                let mem_ptr = ((memory_slice[1] as u16) << 8) | memory_slice[0] as u16;
                if is_reg_target {
                    Ok(Self::MovAccMem(MovMemOperand::Register(register), MovMemOperand::MemoryPtr(mem_ptr)))
                } else {
                    Ok(Self::MovAccMem(MovMemOperand::MemoryPtr(mem_ptr), MovMemOperand::Register(register)))
                }
            }
            Opcode::PUSH => {
                Ok(Self::Push(Register::from_register_code(opcode_byte & 0b00000111, false)?))
            }
            Opcode::POP => {
                Ok(Self::Pop(Register::from_register_code(opcode_byte & 0b00000111, false)?))
            }
            Opcode::ADD => {
                let operands = decode_operands_from_mod_rm_opcode(opcode_byte, memory_slice)?;
                Ok(Self::Add(operands.0, operands.1))
            }
            Opcode::ADD_ACC_8 => {
                Ok(Self::AddAcc8(memory_slice[0]))
            }
            Opcode::ADD_ACC_16 => {
                let val = (memory_slice[1] as u16) << 8 | memory_slice[0] as u16;
                Ok(Self::AddAcc16(val))
            }
        }
    }

    pub fn get_instr_size(&self) -> u16 {
        match self {
            Self::Noop => 1,
            Self::MovImm8(..) => 2,
            Self::MovImm16(..) => 3,
            Self::Mov(operand1, operand2) => 2 +
                if let Operand::Memory(mem_add) = operand1 {
                    mem_add.displacement_size as u16
                } else {
                    0
                } + if let Operand::Memory(mem_add) = operand2 {
                    mem_add.displacement_size as u16
                } else {
                    0
                },
            Self::MovAccMem(_, _) => 3,
            Self::Push(_) | Self::Pop(_) => 1,
            Self::Add(operand1, operand2) => 2 +
                if let Operand::Memory(mem_add) = operand1 {
                    mem_add.displacement_size as u16
                } else {
                    0
                } + if let Operand::Memory(mem_add) = operand2 {
                mem_add.displacement_size as u16
            } else {
                0
            },
            Self::AddAcc8(_) => 2,
            Self::AddAcc16(_) => 3,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::NOOP as u8 => Ok(Self::NOOP),
            x if x >= 0x50 && x <= 0x57 => Ok(Self::PUSH),
            x if x >= 0x58 && x <= 0x5F => Ok(Self::POP),
            x if x >= 0xB0 && x <= 0xBF => Ok(Self::MOV_IMM),
            x if x >= 0x88 && x <= 0x8B => Ok(Self::MOV_REG_MEM),
            x if x >= 0xA0 && x <= 0xA3 => Ok(Self::MOV_ACC_MEM),
            x if /* x >= 0x00 && */ x <= 0x03 => Ok(Self::ADD),
            x if x == Self::ADD_ACC_8 as u8 => Ok(Self::ADD_ACC_8),
            x if x == Self::ADD_ACC_16 as u8 => Ok(Self::ADD_ACC_16),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}