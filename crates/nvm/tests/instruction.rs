use nvm::instruction::{Instruction, MemAddress, MovOperand, Opcode};
use nvm::register::Register;

#[test]
fn test_opcode_from_byte() {
    let noop_opcode = Opcode::try_from(0x90).unwrap();
    assert_eq!(noop_opcode, Opcode::NOOP);

    for x in 0xB0..=0xBF {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV);
    }

    for x in 0x88..=0x8B {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV_REG);
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
        if x >= 0x88 && x <= 0x8B {
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

    let instr = Instruction::MovImm8(Register::AH, 0xFF);
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::MovImm16(Register::AX, 0xFFFF);
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::Mov(MovOperand::Register(Register::AX), MovOperand::Memory(MemAddress::default()));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mov(MovOperand::Register(Register::AX), MovOperand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Mov(MovOperand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }), MovOperand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Mov(MovOperand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }), MovOperand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 7);
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
        if x >= 0x88 && x <= 0x8B {
            continue;
        }

        let result = Instruction::from_bytes(x, &[]);
        assert!(result.is_err())
    }
}

#[test]
fn test_noop_instruction_from_bytes() {
    let instr = Instruction::from_bytes(Opcode::NOOP as u8, &[]).unwrap();
    assert_eq!(instr, Instruction::Noop);
}

#[test]
fn test_mov_imm_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xB0, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::MovImm8(Register::AL, 0xFF));
    let instr = Instruction::from_bytes(0xB8, &[0xFF, 0xFF]).unwrap();
    assert_eq!(instr, Instruction::MovImm16(Register::AX, 0xFFFF));
}

#[test]
fn test_mov_reg_reg_instruction_from_bytes() {

    // ===========================
    // MOV REG8, REG8
    // ===========================
    {

        // MOV BL, CL
        // 88 - r/m8 <- r8
        //    11 001 011
        //    MD CL  BL
        // BL(target in ModRMByte) = CL(reg)
        let instr = Instruction::from_bytes(0x88, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::BL), MovOperand::Register(Register::CL)));
        // MOV BL, CL
        // 8A - r8 <- r/m8
        //    11 011 001
        //    MD BL  CL
        // BL(reg) = CL(in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11011001]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::BL), MovOperand::Register(Register::CL)));
        // MOV CL, BL
        // 8A - r8 <- r/m8, reversed order, 88 is r/m8 <- r8
        //    11 001 011
        //    MD CL  BL
        // CL(reg) = BL(target in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::CL), MovOperand::Register(Register::BL)));
    }

    // ===========================
    // MOV REG16, REG16
    // ===========================
    {

        // MOV BX, CX
        // 89 - r/m16 <- r16
        //    11 001 011
        //    MD CX  BX
        let instr = Instruction::from_bytes(0x89, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::BX), MovOperand::Register(Register::CX)));
        // MOV BX, CX
        // 8B - r16 <- r/m16
        //    11 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b11011001]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::BX), MovOperand::Register(Register::CX)));
        // MOV CX, BX
        // 8B - r16 <- r/m16, reversed order, 89 is r/m16 <- r16
        //    11 001 011
        //    MD CX  BX
        let instr = Instruction::from_bytes(0x8B, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::CX), MovOperand::Register(Register::BX)));
    }
}

#[test]
fn test_mov_reg_mem_instruction_from_bytes() {

    // ===========================
    // MOV REG8, [REG8]
    // ===========================
    {

        // MOV [BP + DI], CL
        // 88 - r/m8 <- r8
        //    00 001 011
        //    MD CL  BL
        let instr = Instruction::from_bytes(0x88, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), MovOperand::Register(Register::CL)));

        // MOV CL, [BP + DI]
        // 8A - r8 <- r/m8
        //    00 011 001
        //    MD BL  CL
        let instr = Instruction::from_bytes(0x8A, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::CL), MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        })));
    }

    // ===========================
    // MOV REG16, [REG16]
    // ===========================
    {

        // MOV [BP + DI], CX
        // 89 0B        r/m16 <- r16
        //    00 001 011
        //    MD CX  BX
        let instr = Instruction::from_bytes(0x89, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), MovOperand::Register(Register::CX)));

        // MOV CX, [BP + DI]
        // 8B 19        r16 <- r/m16
        //    00 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Register(Register::CX), MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        })));
    }
}

#[test]
fn test_mov_reg_mem_mod_0_instruction_from_bytes() {
    for mov_opcode in [0x88, 0x89] {

        let reg_operand = if mov_opcode == 0x88 {
            MovOperand::Register(Register::CL)
        } else {
            MovOperand::Register(Register::CX)
        };

        //MOD = 00, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001000]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 001 -> [BX + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001001]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 010 -> [BP + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001010]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 011 -> [BP + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 100 -> [SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001100]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: None,
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 100 -> [DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001101]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: None,
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 110 -> [displacement16]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001110, 0xAA, 0xBB]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xBBAA,
            displacement_size: 2,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 4);

        //MOD = 00, RM = 111 -> [BX]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001111]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BX),
            index: None,
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));
    }
}
#[test]
fn test_mov_reg_mem_displacement_instruction_from_bytes() {
    for mov_opcode in [0x88, 0x89] {

        let reg_operand = if mov_opcode == 0x88 {
            MovOperand::Register(Register::CL)
        } else {
            MovOperand::Register(Register::CX)
        };

        //MOD = 01 -> displacement size = 1, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b01001000, 0xAA]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAA,
            displacement_size: 1,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 3);

        //MOD = 10 -> displacement size = 2, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b10001000, 0xAA, 0xBB]).unwrap();
        assert_eq!(instr, Instruction::Mov(MovOperand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xBBAA,
            displacement_size: 2,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 4);
    }
}