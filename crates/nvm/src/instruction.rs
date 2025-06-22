use crate::modrm::{decode_operand_from_single_mod_rm_opcode, decode_operands_from_mod_rm_opcode, Operand};
use crate::register::Register;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Opcode {
    NOOP = 0x90, // NOP
    PUSH, // 50 - 57, PUSH reg
    POP,  // 58 - 5F, POP reg
    MOV_IMM = 0xB0, // B0 - BF, MOV reg, imm8/16 (immediate to register)
    MOV_REG_MEM = 0x88, // 88 - 8B, MOV r/m, r || MOV r, r/m
    MOV_ACC_MEM = 0xA0, // A0 - A3, MOV AL/AX <-> [imm16]
    ADD = 0x00, // 00 - 03, ADD r/m, r || ADD r, r/m
    ADD_ACC_8 = 0x04, // ADD AL, imm8
    ADD_ACC_16 = 0x05, // ADD AX, imm16
    SUB = 0x28, // 28 - 2B, SUB r/m, r || SUB r, r/m
    SUB_ACC_8 = 0x2C, // SUB AL, imm8
    SUB_ACC_16 = 0x2D, // SUB AX, imm16
    INC = 0x40, // 40 - 47, INC r
    DEC = 0x48, // 48 - 4F, DEC r
    MUL_DIV_8 = 0xF6, // MUL/DIV BYTE r/m ----- DIV if reg part (bit 2-4) of r/m is 0b110
    MUL_DIV_16 = 0xF7, // MUL/DIV BYTE r/m ----- DIV if reg part (bit 2-4) of r/m is 0b110

    AND = 0x20, // 20 - 23, AND r/m, r || AND r, r/m
    AND_ACC_8 = 0x24, // AND AL, imm8
    AND_ACC_16 = 0x25, // AND AX, imm16
    OR = 0x08, // 08 - 0B, OR r/m, r || OR r, r/m
    OR_ACC_8 = 0x0C, // OR AL, imm8
    OR_ACC_16 = 0x0D, // OR AX, imm16

    JMP = 0xE9,
    JMP_FAR = 0xEA,
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
    Sub(Operand, Operand),
    SubAcc8(u8),
    SubAcc16(u16),
    Inc(Register),
    Dec(Register),
    Mul8(Operand),
    Mul16(Operand),
    Div8(Operand),
    Div16(Operand),
    And(Operand, Operand),
    AndAcc8(u8),
    AndAcc16(u16),
    Or(Operand, Operand),
    OrAcc8(u8),
    OrAcc16(u16),
    JmpNear(i16),
    JmpFar(u16, u16),
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
            Opcode::SUB => {
                let operands = decode_operands_from_mod_rm_opcode(opcode_byte, memory_slice)?;
                Ok(Self::Sub(operands.0, operands.1))
            }
            Opcode::SUB_ACC_8 => {
                Ok(Self::SubAcc8(memory_slice[0]))
            }
            Opcode::SUB_ACC_16 => {
                let val = (memory_slice[1] as u16) << 8 | memory_slice[0] as u16;
                Ok(Self::SubAcc16(val))
            }
            Opcode::INC => {
                let reg_bits = opcode_byte & 0b00000111;
                Ok(Self::Inc(Register::from_register_code(reg_bits, false)?))
            }
            Opcode::DEC => {
                let reg_bits = opcode_byte & 0b00000111;
                Ok(Self::Dec(Register::from_register_code(reg_bits, false)?))
            }
            Opcode::AND => {
                let operands = decode_operands_from_mod_rm_opcode(opcode_byte, memory_slice)?;
                Ok(Self::And(operands.0, operands.1))
            }
            Opcode::AND_ACC_8 => {
                Ok(Self::AndAcc8(memory_slice[0]))
            }
            Opcode::AND_ACC_16 => {
                let val = (memory_slice[1] as u16) << 8 | memory_slice[0] as u16;
                Ok(Self::AndAcc16(val))
            }
            Opcode::OR => {
                let operands = decode_operands_from_mod_rm_opcode(opcode_byte, memory_slice)?;
                Ok(Self::Or(operands.0, operands.1))
            }
            Opcode::OR_ACC_8 => {
                Ok(Self::OrAcc8(memory_slice[0]))
            }
            Opcode::OR_ACC_16 => {
                let val = (memory_slice[1] as u16) << 8 | memory_slice[0] as u16;
                Ok(Self::OrAcc16(val))
            }
            Opcode::MUL_DIV_8 => {
                let operand = decode_operand_from_single_mod_rm_opcode(memory_slice, true)?;
                if memory_slice[0] & 0b00111000 == 0b00110000 {
                    Ok(Self::Div8(operand))
                } else {
                    Ok(Self::Mul8(operand))
                }
            }
            Opcode::MUL_DIV_16 => {
                let operand = decode_operand_from_single_mod_rm_opcode(memory_slice, false)?;
                if memory_slice[0] & 0b00111000 == 0b00110000 {
                    Ok(Self::Div16(operand))
                } else {
                    Ok(Self::Mul16(operand))
                }
            }
            Opcode::JMP => {
                let offset = (memory_slice[1] as i16) << 8 | memory_slice[0] as i16;
                Ok(Self::JmpNear(offset))
            }
            Opcode::JMP_FAR => {
                let offset = (memory_slice[1] as u16) << 8 | memory_slice[0] as u16;
                let segment = (memory_slice[3] as u16) << 8 | memory_slice[2] as u16;
                Ok(Self::JmpFar(segment, offset))
            }
        }
    }

    pub fn get_instr_size(&self) -> u16 {
        match self {
            Self::Noop | Self::Push(_) | Self::Pop(_) | Self::Inc(_) | Self::Dec(_) => 1,
            Self::MovImm8(..) | Self::AddAcc8(_) | Self::SubAcc8(_) | Self::AndAcc8(_) | Self::OrAcc8(_) => 2,
            Self::MovImm16(..) | Self::MovAccMem(_, _) | Self::AddAcc16(_) | Self::SubAcc16(_)
            | Self::AndAcc16(_) | Self::OrAcc16(_) => 3,
            Self::Mov(operand1, operand2)
            | Self::Add(operand1, operand2)
            | Self::Sub(operand1, operand2)
            | Self::And(operand1, operand2)
            | Self::Or(operand1, operand2) => 2 +
                if let Operand::Memory(mem_add) = operand1 {
                    mem_add.displacement_size as u16
                } else {
                    0
                } + if let Operand::Memory(mem_add) = operand2 {
                mem_add.displacement_size as u16
            } else {
                0
            },
            Self::Mul8(operand) | Self::Mul16(operand)
            | Self::Div8(operand) | Self::Div16(operand) => 2 +
                if let Operand::Memory(mem_add) = operand {
                    mem_add.displacement_size as u16
                } else {
                    0
                },
            Self::JmpNear(_) | Self::JmpFar(..) => unreachable!(),
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
            x if x >= 0x28 && x <= 0x2B => Ok(Self::SUB),
            x if x == Self::SUB_ACC_8 as u8 => Ok(Self::SUB_ACC_8),
            x if x == Self::SUB_ACC_16 as u8 => Ok(Self::SUB_ACC_16),
            x if x >= 0x40 && x <= 0x47 => Ok(Self::INC),
            x if x >= 0x48 && x <= 0x4F => Ok(Self::DEC),
            x if x == Self::MUL_DIV_8 as u8 => Ok(Self::MUL_DIV_8),
            x if x == Self::MUL_DIV_16 as u8 => Ok(Self::MUL_DIV_16),
            x if x >= 0x20 && x <= 0x23 => Ok(Self::AND),
            x if x == Self::AND_ACC_8 as u8 => Ok(Self::AND_ACC_8),
            x if x == Self::AND_ACC_16 as u8 => Ok(Self::AND_ACC_16),
            x if x >= 0x08 && x <= 0x0B => Ok(Self::OR),
            x if x == Self::OR_ACC_8 as u8 => Ok(Self::OR_ACC_8),
            x if x == Self::OR_ACC_16 as u8 => Ok(Self::OR_ACC_16),
            x if x == Self::JMP as u8 => Ok(Self::JMP),
            x if x == Self::JMP_FAR as u8 => Ok(Self::JMP_FAR),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}