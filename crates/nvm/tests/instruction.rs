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

    for x in 0x40..=0x47 {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::INC);
    }

    for x in 0x48..=0x4F {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::DEC);
    }

    for x in 0x20..=0x23 {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::AND);
    }

    let noop_opcode = Opcode::try_from(0x24).unwrap();
    assert_eq!(noop_opcode, Opcode::AND_ACC_8);

    let noop_opcode = Opcode::try_from(0x25).unwrap();
    assert_eq!(noop_opcode, Opcode::AND_ACC_16);

    for x in 0x08..=0x0B {
        let mov_opcode = Opcode::try_from(x).unwrap();
        assert_eq!(mov_opcode, Opcode::OR);
    }

    let noop_opcode = Opcode::try_from(0x0C).unwrap();
    assert_eq!(noop_opcode, Opcode::OR_ACC_8);

    let noop_opcode = Opcode::try_from(0x0D).unwrap();
    assert_eq!(noop_opcode, Opcode::OR_ACC_16);

    let noop_opcode = Opcode::try_from(0xF6).unwrap();
    assert_eq!(noop_opcode, Opcode::MUL_DIV_8);

    let noop_opcode = Opcode::try_from(0xF7).unwrap();
    assert_eq!(noop_opcode, Opcode::MUL_DIV_16);

    let noop_opcode = Opcode::try_from(0xE9).unwrap();
    assert_eq!(noop_opcode, Opcode::JMP);

    let noop_opcode = Opcode::try_from(0xEA).unwrap();
    assert_eq!(noop_opcode, Opcode::JMP_FAR);

    let noop_opcode = Opcode::try_from(0xEB).unwrap();
    assert_eq!(noop_opcode, Opcode::JMP_SHORT);

    let noop_opcode = Opcode::try_from(0x74).unwrap();
    assert_eq!(noop_opcode, Opcode::JZ);

    let noop_opcode = Opcode::try_from(0x75).unwrap();
    assert_eq!(noop_opcode, Opcode::JNZ);
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
        if
        /* x >= 0x00 && */
        x <= 0x03 {
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
        if x >= 0x40 && x <= 0x4F {
            continue;
        }
        if x >= 0x20 && x <= 0x23 {
            continue;
        }
        if x == Opcode::AND_ACC_8 as u8 {
            continue;
        }
        if x == Opcode::AND_ACC_16 as u8 {
            continue;
        }
        if x >= 0x08 && x <= 0x0B {
            continue;
        }
        if x == Opcode::OR_ACC_8 as u8 {
            continue;
        }
        if x == Opcode::OR_ACC_16 as u8 {
            continue;
        }
        if x == Opcode::MUL_DIV_8 as u8 {
            continue;
        }
        if x == Opcode::MUL_DIV_16 as u8 {
            continue;
        }
        if x == Opcode::JMP as u8 {
            continue;
        }
        if x == Opcode::JMP_FAR as u8 {
            continue;
        }
        if x == Opcode::JMP_SHORT as u8 {
            continue;
        }
        if x == Opcode::JZ as u8 {
            continue;
        }
        if x == Opcode::JNZ as u8 {
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
    let instr = Instruction::Mov(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress::default()),
    );
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mov(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Mov(
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        Operand::Register(Register::AX),
    );
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Mov(
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::MovAccMem(
        MovMemOperand::MemoryPtr(0),
        MovMemOperand::Register(Register::AL),
    );
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
    let instr = Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress::default()),
        false,
    );
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        false,
    );
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Add(
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        Operand::Register(Register::AX),
        false,
    );
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Add(
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        false,
    );
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::AddAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::AddAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==      SUB      ==
    // ===================
    let instr = Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress::default()),
        false,
    );
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        false,
    );
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Sub(
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        Operand::Register(Register::AX),
        false,
    );
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Sub(
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        false,
    );
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::SubAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::SubAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==   INC & DEC   ==
    // ===================
    let instr = Instruction::Inc(Register::AX);
    assert_eq!(instr.get_instr_size(), 1);

    let instr = Instruction::Dec(Register::AX);
    assert_eq!(instr.get_instr_size(), 1);

    // ===================
    // ==      AND      ==
    // ===================
    let instr = Instruction::And(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress::default()),
    );
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::And(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::And(
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        Operand::Register(Register::AX),
    );
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::And(
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::AndAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::AndAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==      OR       ==
    // ===================
    let instr = Instruction::Or(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress::default()),
    );
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Or(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Or(
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
        Operand::Register(Register::AX),
    );
    assert_eq!(instr.get_instr_size(), 5);
    let instr = Instruction::Or(
        Operand::Memory(MemAddress {
            displacement_size: 2,
            ..Default::default()
        }),
        Operand::Memory(MemAddress {
            displacement_size: 3,
            ..Default::default()
        }),
    );
    assert_eq!(instr.get_instr_size(), 7);

    let instr = Instruction::OrAcc8(0);
    assert_eq!(instr.get_instr_size(), 2);

    let instr = Instruction::OrAcc16(0);
    assert_eq!(instr.get_instr_size(), 3);

    // ===================
    // ==      MUL      ==
    // ===================

    let instr = Instruction::Mul8(Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mul8(Operand::Memory(MemAddress {
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mul8(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 5);

    let instr = Instruction::Mul16(Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mul16(Operand::Memory(MemAddress {
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Mul16(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 5);

    // ===================
    // ==      DIV      ==
    // ===================

    let instr = Instruction::Div8(Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Div8(Operand::Memory(MemAddress {
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Div8(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 5);

    let instr = Instruction::Div16(Operand::Register(Register::AX));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Div16(Operand::Memory(MemAddress {
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Div16(Operand::Memory(MemAddress {
        displacement_size: 3,
        ..Default::default()
    }));
    assert_eq!(instr.get_instr_size(), 5);

    // ===================
    // ==     JUMP      ==
    // ===================

    let instr = Instruction::JmpShort(0);
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::JmpNear(0);
    assert_eq!(instr.get_instr_size(), 3);
    let instr = Instruction::JmpFar(0, 0);
    assert_eq!(instr.get_instr_size(), 4);
    let instr = Instruction::Jz(0);
    assert_eq!(instr.get_instr_size(), 2);
    let instr = Instruction::Jnz(0);
    assert_eq!(instr.get_instr_size(), 2);
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::BL),
                Operand::Register(Register::CL)
            )
        );
        // MOV BL, CL
        // 8A - r8 <- r/m8
        //    11 011 001
        //    MD BL  CL
        // BL(reg) = CL(in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11011001]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::BL),
                Operand::Register(Register::CL)
            )
        );
        // MOV CL, BL
        // 8A - r8 <- r/m8, reversed order, 88 is r/m8 <- r8
        //    11 001 011
        //    MD CL  BL
        // CL(reg) = BL(target in ModRMByte)
        let instr = Instruction::from_bytes(0x8A, &[0b11001011]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::CL),
                Operand::Register(Register::BL)
            )
        );
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::BX),
                Operand::Register(Register::CX)
            )
        );
        // MOV BX, CX
        // 8B - r16 <- r/m16
        //    11 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b11011001]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::BX),
                Operand::Register(Register::CX)
            )
        );
        // MOV CX, BX
        // 8B - r16 <- r/m16, reversed order, 89 is r/m16 <- r16
        //    11 001 011
        //    MD CX  BX
        let instr = Instruction::from_bytes(0x8B, &[0b11001011]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::CX),
                Operand::Register(Register::BX)
            )
        );
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                Operand::Register(Register::CL)
            )
        );

        // MOV CL, [BP + DI]
        // 8A - r8 <- r/m8
        //    00 011 001
        //    MD BL  CL
        let instr = Instruction::from_bytes(0x8A, &[0b00001011]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::CL),
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                })
            )
        );
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                Operand::Register(Register::CX)
            )
        );

        // MOV CX, [BP + DI]
        // 8B 19        r16 <- r/m16
        //    00 011 001
        //    MD BX  CX
        let instr = Instruction::from_bytes(0x8B, &[0b00001011]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Register(Register::CX),
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                })
            )
        );
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BX),
                    index: Some(Register::SI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 001 -> [BX + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001001]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BX),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 010 -> [BP + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001010]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::SI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 011 -> [BP + DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001011]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BP),
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 100 -> [SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001100]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: None,
                    index: Some(Register::SI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 100 -> [DI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001101]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: None,
                    index: Some(Register::DI),
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );

        //MOD = 00, RM = 110 -> [displacement16]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001110, 0xAA, 0xBB]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: None,
                    index: None,
                    displacement: 0xBBAA,
                    displacement_size: 2,
                }),
                reg_operand
            )
        );
        assert_eq!(instr.get_instr_size(), 4);

        //MOD = 00, RM = 111 -> [BX]
        let instr = Instruction::from_bytes(mov_opcode, &[0b00001111]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BX),
                    index: None,
                    displacement: 0,
                    displacement_size: 0,
                }),
                reg_operand
            )
        );
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
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BX),
                    index: Some(Register::SI),
                    displacement: 0xAA,
                    displacement_size: 1,
                }),
                reg_operand
            )
        );
        assert_eq!(instr.get_instr_size(), 3);

        //MOD = 10 -> displacement size = 2, RM = 000 -> [BX + SI]
        let instr = Instruction::from_bytes(mov_opcode, &[0b10001000, 0xAA, 0xBB]).unwrap();
        assert_eq!(
            instr,
            Instruction::Mov(
                Operand::Memory(MemAddress {
                    base: Some(Register::BX),
                    index: Some(Register::SI),
                    displacement: 0xBBAA,
                    displacement_size: 2,
                }),
                reg_operand
            )
        );
        assert_eq!(instr.get_instr_size(), 4);
    }
}

#[test]
fn test_mov_acc_mem_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xA0, &[0xAA, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::MovAccMem(
            MovMemOperand::Register(Register::AL),
            MovMemOperand::MemoryPtr(0xBBAA)
        )
    );
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA1, &[0xAA, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::MovAccMem(
            MovMemOperand::Register(Register::AX),
            MovMemOperand::MemoryPtr(0xBBAA)
        )
    );
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA2, &[0xAA, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::MovAccMem(
            MovMemOperand::MemoryPtr(0xBBAA),
            MovMemOperand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 3);

    let instr = Instruction::from_bytes(0xA3, &[0xAA, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::MovAccMem(
            MovMemOperand::MemoryPtr(0xBBAA),
            MovMemOperand::Register(Register::AX)
        )
    );
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
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AL),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x00, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x01, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AX),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x02, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x00, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x01, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AX),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x02, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x03, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Add(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: None,
                index: None,
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            false
        )
    );
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
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AL),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x28, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x29, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AX),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x2A, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x28, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AL),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x29, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AX),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x2A, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            true
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            false
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x2B, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Sub(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: None,
                index: None,
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            false
        )
    );
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

#[test]
fn test_inc_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x40, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::AX));

    let instr = Instruction::from_bytes(0x41, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::CX));

    let instr = Instruction::from_bytes(0x42, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::DX));

    let instr = Instruction::from_bytes(0x43, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::BX));

    let instr = Instruction::from_bytes(0x44, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::SP));

    let instr = Instruction::from_bytes(0x45, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::BP));

    let instr = Instruction::from_bytes(0x46, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::SI));

    let instr = Instruction::from_bytes(0x47, &[]).unwrap();
    assert_eq!(instr, Instruction::Inc(Register::DI));
}

#[test]
fn test_dec_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x48, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::AX));

    let instr = Instruction::from_bytes(0x49, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::CX));

    let instr = Instruction::from_bytes(0x4A, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::DX));

    let instr = Instruction::from_bytes(0x4B, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::BX));

    let instr = Instruction::from_bytes(0x4C, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::SP));

    let instr = Instruction::from_bytes(0x4D, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::BP));

    let instr = Instruction::from_bytes(0x4E, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::SI));

    let instr = Instruction::from_bytes(0x4F, &[]).unwrap();
    assert_eq!(instr, Instruction::Dec(Register::DI));
}

#[test]
fn test_and_instruction_from_bytes() {
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x20, &[0b11000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AL),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x20, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x21, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AX)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x22, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x23, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x20, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x21, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AX)
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x22, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x23, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x23, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::And(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: None,
                index: None,
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);
}

#[test]
fn test_and_acc_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x24, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::AndAcc8(0xFF));
    assert_eq!(instr.get_instr_size(), 2);
}

#[test]
fn test_and_acc_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x25, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::AndAcc16(0xBBAA));
    assert_eq!(instr.get_instr_size(), 3);
}

#[test]
fn test_or_instruction_from_bytes() {
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x08, &[0b11000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AL),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x08, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x09, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            }),
            Operand::Register(Register::AX)
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x0A, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x0B, &[0b00000000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0,
                displacement_size: 0,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 2);

    // DISPLACEMENT
    // r/m8 <- r8
    let instr = Instruction::from_bytes(0x08, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AL)
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r/m16 <- r16
    let instr = Instruction::from_bytes(0x09, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            }),
            Operand::Register(Register::AX)
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r8 <- r/m8
    let instr = Instruction::from_bytes(0x0A, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AL),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x0B, &[0b10000000, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: Some(Register::BX),
                index: Some(Register::SI),
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);

    // r16 <- r/m16
    let instr = Instruction::from_bytes(0x0B, &[0b00000110, 0xFF, 0xBB]).unwrap();
    assert_eq!(
        instr,
        Instruction::Or(
            Operand::Register(Register::AX),
            Operand::Memory(MemAddress {
                base: None,
                index: None,
                displacement: 0xBBFF,
                displacement_size: 2,
            })
        )
    );
    assert_eq!(instr.get_instr_size(), 4);
}

#[test]
fn test_or_acc_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x0C, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::OrAcc8(0xFF));
}

#[test]
fn test_or_acc_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x0D, &[0xAA, 0xBB]).unwrap();
    assert_eq!(instr, Instruction::OrAcc16(0xBBAA));
}

#[test]
fn test_mul_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xF6, &[0b11001000]).unwrap();
    assert_eq!(instr, Instruction::Mul8(Operand::Register(Register::AL)));

    let instr = Instruction::from_bytes(0xF6, &[0b00001000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b01001000, 0xFF]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xFF,
            displacement_size: 1,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b10001000, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b00001110, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul8(Operand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );
}

#[test]
fn test_mul_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xF7, &[0b11001000]).unwrap();
    assert_eq!(instr, Instruction::Mul16(Operand::Register(Register::AX)));

    let instr = Instruction::from_bytes(0xF7, &[0b00001000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b01001000, 0xFF]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xFF,
            displacement_size: 1,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b10001000, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b00001110, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Mul16(Operand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );
}

#[test]
fn test_div_8_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xF6, &[0b11110000]).unwrap();
    assert_eq!(instr, Instruction::Div8(Operand::Register(Register::AL)));

    let instr = Instruction::from_bytes(0xF6, &[0b00110000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b01110000, 0xFF]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xFF,
            displacement_size: 1,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b10110000, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div8(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );

    let instr = Instruction::from_bytes(0xF6, &[0b00110110, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div8(Operand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );
}

#[test]
fn test_div_16_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xF7, &[0b11110000]).unwrap();
    assert_eq!(instr, Instruction::Div16(Operand::Register(Register::AX)));

    let instr = Instruction::from_bytes(0xF7, &[0b00110000]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b01110000, 0xFF]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xFF,
            displacement_size: 1,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b10110000, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div16(Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );

    let instr = Instruction::from_bytes(0xF7, &[0b00110110, 0xFF, 0xAA]).unwrap();
    assert_eq!(
        instr,
        Instruction::Div16(Operand::Memory(MemAddress {
            base: None,
            index: None,
            displacement: 0xAAFF,
            displacement_size: 2,
        }))
    );
}

#[test]
fn test_jmp_near_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xE9, &[0xFF, 0x0A]).unwrap();
    assert_eq!(instr, Instruction::JmpNear(0x0AFF));

    let instr = Instruction::from_bytes(0xE9, &[0xFF, 0xAA]).unwrap();
    assert_eq!(instr, Instruction::JmpNear(-0x5501));
}

#[test]
fn test_jmp_far_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xEA, &[0xAA, 0xBB, 0xCC, 0xDD]).unwrap();
    assert_eq!(instr, Instruction::JmpFar(0xDDCC, 0xBBAA));
}

#[test]
fn test_jmp_short_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0xEB, &[0x11]).unwrap();
    assert_eq!(instr, Instruction::JmpShort(0x11_i8));

    let instr = Instruction::from_bytes(0xEB, &[0b11111111]).unwrap();
    assert_eq!(instr, Instruction::JmpShort(-1));
}

#[test]
fn test_jz_near_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x74, &[0b01111111]).unwrap();
    assert_eq!(instr, Instruction::Jz(0b01111111));

    let instr = Instruction::from_bytes(0x74, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::Jz(-1));
}

#[test]
fn test_jnz_near_instruction_from_bytes() {
    let instr = Instruction::from_bytes(0x75, &[0b01111111]).unwrap();
    assert_eq!(instr, Instruction::Jnz(0b01111111));

    let instr = Instruction::from_bytes(0x75, &[0xFF]).unwrap();
    assert_eq!(instr, Instruction::Jnz(-1));
}
