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
}

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MemAddress {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub displacement: u16,
    pub displacement_size: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum MovOperand {
    Register(Register),
    Memory(MemAddress),
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
    Mov(MovOperand, MovOperand),
    // MOV AL/AX <-> [imm16]
    //  DEST      , SRC
    MovAccMem(MovMemOperand, MovMemOperand),
    Push(Register),
    Pop(Register),
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
                let is_rm_target = opcode_byte & 0b00000010 == 0; // true if destination should be mod r/m
                let is_8_bit = opcode_byte & 0b00000001 == 0; // true if operating with 8bit registers

                let modrm_byte = memory_slice[0];
                let mod_bits = modrm_byte & 0b11000000;
                let reg_bits = (modrm_byte & 0b00111000) >> 3;
                let rm_bits = modrm_byte & 0b00000111;

                let reg = Register::from_register_code(reg_bits, is_8_bit)?;

                if mod_bits == 0b11000000 {
                    let rm = Register::from_register_code(rm_bits, is_8_bit)?;
                    return if is_rm_target {
                        Ok(Instruction::Mov(MovOperand::Register(rm), MovOperand::Register(reg)))
                    } else {
                        Ok(Instruction::Mov(MovOperand::Register(reg), MovOperand::Register(rm)))
                    }
                }

                let mut displacement_size = mod_bits >> 6;
                let (base_reg, index_reg) = match rm_bits {
                    0b000 => (Some(Register::BX), Some(Register::SI)),
                    0b001 => (Some(Register::BX), Some(Register::DI)),
                    0b010 => (Some(Register::BP), Some(Register::SI)),
                    0b011 => (Some(Register::BP), Some(Register::DI)),
                    0b100 => (None, Some(Register::SI)),
                    0b101 => (None, Some(Register::DI)),
                    x if x == 0b110 && mod_bits == 0 => {
                        displacement_size = 2;
                        (None, None)
                    }
                    0b111 => (Some(Register::BX), None),
                    _ => unreachable!(),
                };

                let displacement = match displacement_size {
                    0 => 0,
                    1 => memory_slice[1] as u16,
                    2 => u16::from_le_bytes([memory_slice[1], memory_slice[2]]),
                    _ => unreachable!(),
                };

                let rm_operand = MovOperand::Memory(MemAddress {
                    base: base_reg,
                    index: index_reg,
                    displacement,
                    displacement_size,
                });

                if is_rm_target {
                    Ok(Self::Mov(rm_operand, MovOperand::Register(reg)))
                } else {
                    Ok(Self::Mov(MovOperand::Register(reg), rm_operand))
                }
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
        }
    }

    pub fn get_instr_size(&self) -> u16 {
        match self {
            Instruction::Noop => 1,
            Instruction::MovImm8(..) => 2,
            Instruction::MovImm16(..) => 3,
            Instruction::Mov(operand1, operand2) => 2 +
                if let MovOperand::Memory(mem_add) = operand1 {
                    mem_add.displacement_size as u16
                } else {
                    0
                } + if let MovOperand::Memory(mem_add) = operand2 {
                    mem_add.displacement_size as u16
                } else {
                    0
                },
            Instruction::MovAccMem(_, _) => 3,
            Instruction::Push(_) | Instruction::Pop(_) => 1,
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
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}