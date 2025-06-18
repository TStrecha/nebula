use nvm::instruction::{Instruction, Opcode};
use nvm::register::Register;

#[test]
fn test_opcode_from_byte() {
    let noop_opcode = Opcode::try_from(0x90).unwrap();
    assert_eq!(noop_opcode, Opcode::NOOP);

    for x in 0xB0..=0xBF {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV);
    }
}

#[test]
fn test_opcode_from_byte_returns_ok_only_for_explicitly_supported_opcodes() {
    for x in 0x00..=0xFF {
        if x == Opcode::NOOP as u8 {
            continue;
        }
        if x >= 0xB0 && x <= 0xBF {
            continue;
        }

        let result = Opcode::try_from(x);
        assert!(result.is_err())
    }
}

#[test]
fn test_opcode_from_invalid_byte() {
    let result = Opcode::try_from(0x00);
    assert!(result.is_err())
}

#[test]
fn test_instruction_get_size() {
    let instr = Instruction::Noop;
    assert_eq!(instr.get_instr_size(), 1);

    let instr = Instruction::Mov8(Register::AH, 0xFF);
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mov16(Register::AX, 0xFFFF);
    assert_eq!(instr.get_instr_size(), 3);
}

#[test]
fn test_instruction_from_bytes() {
    let instr = Instruction::from_bytes(Opcode::NOOP as u8, &[]).unwrap();
    assert_eq!(instr, Instruction::Noop);

    let instr = Instruction::from_bytes(0xB0, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::Mov8(Register::AL, 0xFF));
    let instr = Instruction::from_bytes(0xB8, &[0xFF, 0xFF]).unwrap();
    assert_eq!(instr, Instruction::Mov16(Register::AX, 0xFFFF));
}

#[test]
fn test_instruction_from_invalid_bytes() {
    let result = Instruction::from_bytes(0x00, &[]);
    assert!(result.is_err());
}

#[test]
fn test_instruction_from_byte_returns_ok_only_for_explicitly_supported_opcodes() {
    for x in 0x00..=0xFF {
        if x == Opcode::NOOP as u8 {
            continue;
        }
        if x >= 0xB0 && x <= 0xBF {
            continue;
        }

        let result = Instruction::from_bytes(x, &[]);
        assert!(result.is_err())
    }
}