use crate::register::Register;

#[derive(Debug, Default, PartialEq, Eq, Clone, Copy, Hash)]
pub struct MemAddress {
    pub base: Option<Register>,
    pub index: Option<Register>,
    pub displacement: u16,
    pub displacement_size: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum Operand {
    Register(Register),
    Memory(MemAddress),
}

pub fn decode_operands_from_mod_rm_opcode(opcode_byte: u8, mem_slice: &[u8]) -> Result<(Operand, Operand, bool), String> {
    let is_rm_target = opcode_byte & 0b00000010 == 0; // true if destination should be mod r/m
    let is_8_bit = opcode_byte & 0b00000001 == 0; // true if operating with 8bit registers

    let modrm_byte = mem_slice[0];
    let mod_bits = modrm_byte & 0b11000000;
    let reg_bits = (modrm_byte & 0b00111000) >> 3;
    let rm_bits = modrm_byte & 0b00000111;

    let reg = Register::from_register_code(reg_bits, is_8_bit)?;

    if is_reg_only(mod_bits) {
        let rm = Register::from_register_code(rm_bits, is_8_bit)?;
        return if is_rm_target {
            Ok((Operand::Register(rm), Operand::Register(reg), is_8_bit))
        } else {
            Ok((Operand::Register(reg), Operand::Register(rm), is_8_bit))
        }
    }

    let rm_operand = extract_memory_address_operand(rm_bits, mod_bits, mem_slice);

    if is_rm_target {
        Ok((rm_operand, Operand::Register(reg), is_8_bit))
    } else {
        Ok((Operand::Register(reg), rm_operand, is_8_bit))
    }
}

pub fn decode_operand_from_single_mod_rm_opcode(mem_slice: &[u8], is_8_bit: bool) -> Result<Operand, String> {
    let modrm_byte = mem_slice[0];
    let mod_bits = modrm_byte & 0b11000000;
    let rm_bits = modrm_byte & 0b00000111;

    if is_reg_only(mod_bits) {
        let rm = Register::from_register_code(rm_bits, is_8_bit)?;
        return Ok(Operand::Register(rm))
    }

    Ok(extract_memory_address_operand(rm_bits, mod_bits, mem_slice))
}

pub fn extract_memory_address_operand(rm_bits: u8, mod_bits: u8, mem_slice: &[u8]) -> Operand {
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
        1 => mem_slice[1] as u16,
        2 => u16::from_le_bytes([mem_slice[1], mem_slice[2]]),
        _ => unreachable!(),
    };

    Operand::Memory(MemAddress {
        base: base_reg,
        index: index_reg,
        displacement,
        displacement_size,
    })
}

pub fn is_reg_only(mod_bits: u8) -> bool {
    mod_bits == 0b11000000
}
