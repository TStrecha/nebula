#[repr(C)]
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
pub enum Register {
    // 16bit, sorted:
    AX,
    CX,
    DX,
    BX,

    SP,
    BP,
    SI,
    DI,

    // 16bit, unsorted:
    CS,
    DS,
    SS,
    ES,
    IP,
    F,

    // 8bit, sorted:
    // Low
    AL = 0x80,
    CL = 0x81,
    DL = 0x82,
    BL = 0x83,
    // High
    AH = 0x84,
    CH = 0x85,
    DH = 0x86,
    BH = 0x87,
}

impl Register {
    pub fn is_8bit(&self) -> bool {
        matches!(
            self,
            Register::AL
                | Register::AH
                | Register::BL
                | Register::BH
                | Register::CL
                | Register::CH
                | Register::DL
                | Register::DH
        )
    }

    pub fn from_register_code(code: u8, bits_8: bool) -> Result<Self, String> {
        let code = if bits_8 { code + 0x80 } else { code };

        match code {
            x if x == Self::AX as u8 => Ok(Self::AX),
            x if x == Self::CX as u8 => Ok(Self::CX),
            x if x == Self::DX as u8 => Ok(Self::DX),
            x if x == Self::BX as u8 => Ok(Self::BX),
            x if x == Self::SP as u8 => Ok(Self::SP),
            x if x == Self::BP as u8 => Ok(Self::BP),
            x if x == Self::SI as u8 => Ok(Self::SI),
            x if x == Self::DI as u8 => Ok(Self::DI),
            x if x == Self::CS as u8 => Ok(Self::CS),
            x if x == Self::DS as u8 => Ok(Self::DS),
            x if x == Self::SS as u8 => Ok(Self::SS),
            x if x == Self::ES as u8 => Ok(Self::ES),
            x if x == Self::IP as u8 => Ok(Self::IP),
            x if x == Self::F as u8 => Ok(Self::F),
            x if x == Self::AL as u8 => Ok(Self::AL),
            x if x == Self::CL as u8 => Ok(Self::CL),
            x if x == Self::DL as u8 => Ok(Self::DL),
            x if x == Self::BL as u8 => Ok(Self::BL),
            x if x == Self::AH as u8 => Ok(Self::AH),
            x if x == Self::CH as u8 => Ok(Self::CH),
            x if x == Self::DH as u8 => Ok(Self::DH),
            x if x == Self::BH as u8 => Ok(Self::BH),
            _ => Err(format!("Invalid register code: {}", code)),
        }
    }
}

pub enum Flag {
    CARRY = 0b00000001,
    PARITY = 0b00000100,
    AUXILIARY = 0b00010000,
    ZERO = 0b01000000,
    SIGN = 0b10000000,
    TRAP = 0b00000001_00000000,
    INTERRUPT = 0b00000010_00000000,
    DIRECTION = 0b00000100_00000000,
    OVERFLOW = 0b00001000_00000000,
}

