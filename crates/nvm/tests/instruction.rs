use nvm::instruction::{Instruction, MovMemOperand, Opcode};
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;

#[test]
fn test_opcode_from_byte() {
    let noop_opcode = Opcode::try_from(0x90).unwrap();
    assert_eq!(noop_opcode, Opcode::NOOP);

    for x in 0xB0..=0xBF {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV_IMM);
    }

    for x in 0x88..=0x8B {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV_REG_MEM);
    }

    for x in 0xA0..=0xA3 {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::MOV_ACC_MEM);
    }

    for x in 0x50..=0x57 {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::PUSH);
    }

    for x in 0x58..=0x5F {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::POP);
    }

    for x in 0x00..=0x03 {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::ADD);
    }

    let noop_opcode = Opcode::try_from(0x04).unwrap();
    assert_eq!(noop_opcode, Opcode::ADD_ACC_8);

    let noop_opcode = Opcode::try_from(0x05).unwrap();
    assert_eq!(noop_opcode, Opcode::ADD_ACC_16);

    for x in 0x28..=0x2B {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::SUB);
    }

    let noop_opcode = Opcode::try_from(0x2C).unwrap();
    assert_eq!(noop_opcode, Opcode::SUB_ACC_8);

    let noop_opcode = Opcode::try_from(0x2D).unwrap();
    assert_eq!(noop_opcode, Opcode::SUB_ACC_16);
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
        if x >= 0xA0 && x <= 0xA3 {
            continue;
        }
        if x >= 0x50 && x <= 0x5F {
            continue;
        }
        if /* x >= 0x00 && */ x <= 0x03 {
            continue;
        }
        if x == Opcode::ADD_ACC_8 as u8 {
            continue;
        }
        if x == Opcode::ADD_ACC_16 as u8 {
            continue;
        }
        if x >= 0x28 && x <= 0x2B {
            continue;
        }
        if x == Opcode::SUB_ACC_8 as u8 {
            continue;
        }
        if x == Opcode::SUB_ACC_16 as u8 {
            continue;
        }

        let result = Opcode::try_from(x);
        assert!(result.is_err())
    }
}

#[test]
fn test_opcode_from_invalid_byte() {
    let result = Opcode::try_from(0xFF);
    assert!(result.is_err())
}

#[test]
fn test_instruction_get_size() {
    // ===================
    // ==      NOOP     ==
    // ===================
    let instr = Instruction::Noop;
    assert_eq!(instr.get_instr_size(), 1);

    // ===================
    // ==    MOV IMM    ==
    // ===================
    let instr = Instruction::MovImm8(Register::AH, 0xFF);
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::MovImm16(Register::AX, 0xFFFF);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==      MOV      ==
    // ===================
    let instr = Instruction::Mov(Operand::Register(Register::AX), Operand::Memory(MemAddress::default()));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mov(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Mov(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }), Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Mov(Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }), Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::MovAccMem(MovMemOperand::MemoryPtr(0), MovMemOperand::Register(Register::AL));
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==   PUSH & POP  ==
    // ===================
    let instr = Instruction::Push(Register::AL);
    assert_eq!(instr.get_instr_size(), 1);

    let instr = Instruction::Pop(Register::AL);
    assert_eq!(instr.get_instr_size(), 1);

    // ===================
    // ==      ADD      ==
    // ===================
    let instr = Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress::default()));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Add(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }), Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Add(Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }), Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::AddAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::AddAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==      SUB      ==
    // ===================
    let instr = Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress::default()));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Sub(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }), Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Sub(Operand::Memory(MemAddress {
        displacement_size: 2,
        ..Default::default()
    }), Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::SubAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::SubAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);
}

#[test]
fn test_instruction_from_invalid_bytes() {
    let result = Instruction::from_bytes(0xFF, &[]);
    assert!(result.is_err());
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
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::BL), Operand::Register(Register::CL)));
        // MOV BL, CL
        // 8A - r8 <- r/m8
        //    11 011 001
        //    MD BL  CL
        // BL(reg) = CL(in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11011001]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::BL), Operand::Register(Register::CL)));
        // MOV CL, BL
        // 8A - r8 <- r/m8, reversed order, 88 is r/m8 <- r8
        //    11 001 011
        //    MD CL  BL
        // CL(reg) = BL(target in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::CL), Operand::Register(Register::BL)));
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
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::BX), Operand::Register(Register::CX)));
        // MOV BX, CX
        // 8B - r16 <- r/m16
        //    11 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b11011001]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::BX), Operand::Register(Register::CX)));
        // MOV CX, BX
        // 8B - r16 <- r/m16, reversed order, 89 is r/m16 <- r16
        //    11 001 011
        //    MD CX  BX
        let instr = Instruction::from_bytes(0x8B, &[0b11001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::CX), Operand::Register(Register::BX)));
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
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), Operand::Register(Register::CL)));

        // MOV CL, [BP + DI]
        // 8A - r8 <- r/m8
        //    00 011 001
        //    MD BL  CL
        let instr = Instruction::from_bytes(0x8A, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
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
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), Operand::Register(Register::CX)));

        // MOV CX, [BP + DI]
        // 8B 19        r16 <- r/m16
        //    00 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
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
            Operand::Register(Register::CL)
        } else {
            Operand::Register(Register::CX)
        };

        //MOD = 00, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001000]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 001 -> [BX + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001001]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 010 -> [BP + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001010]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 011 -> [BP + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001011]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BP),
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 100 -> [SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001100]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: None,
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 100 -> [DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001101]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: None,
            index: Some(Register::DI),
            displacement: 0,
            displacement_size: 0,
        }), reg_operand));

        //MOD = 00, RM = 110 -> [displacement16]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001110, 0xAA, 0xBB]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xBBAA,
            displacement_size: 2,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 4);

        //MOD = 00, RM = 111 -> [BX]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001111]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
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
            Operand::Register(Register::CL)
        } else {
            Operand::Register(Register::CX)
        };

        //MOD = 01 -> displacement size = 1, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b01001000, 0xAA]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAA,
            displacement_size: 1,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 3);

        //MOD = 10 -> displacement size = 2, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b10001000, 0xAA, 0xBB]).unwrap();
        assert_eq!(instr, Instruction::Mov(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xBBAA,
            displacement_size: 2,
        }), reg_operand));
        assert_eq!(instr.get_instr_size(), 4);
    }
}

#[test]
fn test_mov_acc_mem_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xA0, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::MovAccMem(MovMemOperand::Register(Register::AL), MovMemOperand::MemoryPtr(0xBBAA)));
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA1, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::MovAccMem(MovMemOperand::Register(Register::AX), MovMemOperand::MemoryPtr(0xBBAA)));
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA2, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::MovAccMem(MovMemOperand::MemoryPtr(0xBBAA), MovMemOperand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA3, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::MovAccMem(MovMemOperand::MemoryPtr(0xBBAA), MovMemOperand::Register(Register::AX)));
    assert_eq!(instr.get_instr_size(), 3);
}

#[test]
fn test_push_pop_instruction_from_bytes() {

    // ===========================
    // PUSH
    // ===========================
    {
        let instr = Instruction::from_bytes(0x50, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::AX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x51, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::CX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x52, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::DX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x53, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::BX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x54, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::SP));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x55, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::BP));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x56, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::SI));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x57, &[]).unwrap();
        assert_eq!(instr, Instruction::Push(Register::DI));
        assert_eq!(instr.get_instr_size(), 1);
    }

    // ===========================
    // POP
    // ===========================
    {
        let instr = Instruction::from_bytes(0x58, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::AX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x59, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::CX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5A, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::DX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5B, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::BX));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5C, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::SP));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5D, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::BP));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5E, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::SI));
        assert_eq!(instr.get_instr_size(), 1);

        let instr = Instruction::from_bytes(0x5F, &[]).unwrap();
        assert_eq!(instr, Instruction::Pop(Register::DI));
        assert_eq!(instr.get_instr_size(), 1);
    }
}

#[test]
fn test_add_instruction_from_bytes() {
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x00, &[0b11000000]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AL), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x00, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x01, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::AX)));
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x02, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x00, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    }), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x01, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    }), Operand::Register(Register::AX)));
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x02, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: None,
        index: None,
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);
}

#[test]
fn test_add_acc_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x04, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::AddAcc8(0xFF));
    assert_eq!(instr.get_instr_size(), 2);
}

#[test]
fn test_add_acc_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x05, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::AddAcc16(0xBBAA));
    assert_eq!(instr.get_instr_size(), 3);
}

#[test]
fn test_sub_instruction_from_bytes() {
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x28, &[0b11000000]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AL), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x28, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x29, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::AX)));
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x2A, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b00000000]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x28, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    }), Operand::Register(Register::AL)));
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x29, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    }), Operand::Register(Register::AX)));
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x2A, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: None,
        index: None,
        displacement: 0xBBFF,
        displacement_size: 2,
    })));
    assert_eq!(instr.get_instr_size(), 4);
}

#[test]
fn test_sub_acc_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x2C, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::SubAcc8(0xFF));
    assert_eq!(instr.get_instr_size(), 2);
}

#[test]
fn test_sub_acc_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x2D, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::SubAcc16(0xBBAA));
    assert_eq!(instr.get_instr_size(), 3);
}