use crate::register::Register;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Opcode {
    NOOP = 0x90,
    MOV = 0xB0, //B0 - BF
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Instruction {
    Noop,
    Mov8(Register, u8),
    Mov16(Register, u16),
}

impl Instruction {
    pub fn from_bytes(opcode_byte: u8, memory_slice: &[u8]) -> Result<Self, String> {
        let opcode = Opcode::try_from(opcode_byte)?;

        match opcode {
            Opcode::NOOP => Ok(Self::Noop),
            Opcode::MOV => {
                let bits_8 = (opcode_byte & 0b00001000) == 0;
                let reg_bits = opcode_byte & 0b00000111;
                let reg = Register::from_register_code(reg_bits, bits_8)?;

                let instr = if bits_8 {
                    let val = memory_slice[0];
                    Self::Mov8(reg, val)
                } else {
                    let upper = memory_slice[1] as u16;
                    let lower = memory_slice[0] as u16;
                    Self::Mov16(reg, (upper << 8) | lower)
                };

                Ok(instr)
            }
        }
    }

    pub fn get_instr_size(&self) -> u16 {
        match self {
            Instruction::Noop => 1,
            Instruction::Mov8(..) => 2,
            Instruction::Mov16(..) => 3,
        }
    }
}

impl TryFrom<u8> for Opcode {
    type Error = String;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::NOOP as u8 => Ok(Self::NOOP),
            x if x >= 0xB0 && x <= 0xBF => Ok(Self::MOV),
            _ => Err(format!("Invalid opcode: {:#x}", value)),
        }
    }
}