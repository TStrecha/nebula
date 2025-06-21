use nvm::instruction::{Instruction, MovMemOperand};
use nvm::Machine;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;
use nvm_test_utils::machine_test;

#[machine_test]
fn test_mov_8bit(mut machine: Machine) {
    // MOV AL, 0xFF
    // MOV AH, 0x10
    machine.run_instruction(Instruction::MovImm8(Register::AL, 0xFF));
    machine.run_instruction(Instruction::MovImm8(Register::AH, 0x10));

    assert_eq!(machine.get_register(Register::AL), 0xFF);
    assert_eq!(machine.get_register(Register::AH), 0x10);

    assert_eq!(machine.get_register(Register::AX), 0x10FF);
}

#[machine_test]
fn test_mov_16bit(mut machine: Machine) {
    // MOV AX, 0xFF10
    machine.run_instruction(Instruction::MovImm16(Register::AX, 0xFF10));

    assert_eq!(machine.get_register(Register::AX), 0xFF10);
}

#[machine_test]
#[machine_state(Register::CL = 0xAA)]
#[machine_state(Register::DL = 0xBB)]
#[machine_state(Register::CX = 0xAAAA)]
#[machine_state(Register::DX = 0xBBBB)]
fn test_mov_reg_to_reg(mut machine: Machine) {
    // 8 bit
    // MOV BL, CL
    // MOV AL, DL
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::BL), Operand::Register(Register::CL)));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::AL), Operand::Register(Register::DL)));

    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.get_register(Register::BL), 0xAA);
    assert_eq!(machine.get_register(Register::DL), 0xBB);
    assert_eq!(machine.get_register(Register::AL), 0xBB);

    // 16 bit
    // MOV BX, CX
    // MOV AX, DX
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::BX), Operand::Register(Register::CX)));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::AX), Operand::Register(Register::DX)));

    assert_eq!(machine.get_register(Register::CX), 0xAAAA);
    assert_eq!(machine.get_register(Register::BX), 0xAAAA);
    assert_eq!(machine.get_register(Register::DX), 0xBBBB);
    assert_eq!(machine.get_register(Register::AX), 0xBBBB);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CL = 0xAA)]
fn test_mov_8bit_reg_to_mem(mut machine: Machine) {
    // MOV [BX + SI], CL
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CX = 0xAABB)]
fn test_mov_16bit_reg_to_mem(mut machine: Machine) {
    // MOV [BX + SI], CX
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    }), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 1], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CL = 0xAA)]
fn test_mov_8bit_reg_to_mem_with_1byte_displacement(mut machine: Machine) {
    // MOV [BX + SI + 0xC], CL
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x0C,
        displacement_size: 1,
    }), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CX = 0xAABB)]
fn test_mov_16bit_reg_to_mem_with_1byte_displacement(mut machine: Machine) {
    // MOV [BX + SI + 0xC], CX
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x0C,
        displacement_size: 1,
    }), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC + 1], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CL = 0xAA)]
fn test_mov_8bit_reg_to_mem_with_2byte_displacement(mut machine: Machine) {
    // MOV [BX + SI + 0xD0C], CL
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(Register::CX = 0xAABB)]
fn test_mov_16bit_reg_to_mem_with_2byte_displacement(mut machine: Machine) {
    // MOV [BX + SI + 0xD0C], CX
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB = 0xAA)]
fn test_mov_8bit_mem_to_reg(mut machine: Machine) {
    // MOV CL, [BX + SI]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB = 0xAA)]
#[machine_state(0xA + 0xB + 1   = 0xBB)]
fn test_mov_16bit_mem_to_reg(mut machine: Machine) {
    // MOV CX, [BX + SI]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB + 0xC = 0xAA)]
fn test_mov_8bit_mem_to_reg_with_1byte_displacement(mut machine: Machine) {
    // MOV CL, [BX + SI + 0xC]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x0C,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB + 0xC = 0xAA)]
#[machine_state(0xA + 0xB + 0xC + 1 = 0xBB)]
fn test_mov_16bit_mem_to_reg_1byte_displacement(mut machine: Machine) {
    // MOV CX, [BX + SI + 0xC]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x0C,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB + 0xD0C = 0xAA)]
fn test_mov_8bit_mem_to_reg_with_2byte_displacement(mut machine: Machine) {
    // MOV CL, [BX + SI + 0xD0C]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[machine_test]
#[machine_state(Register::BX = 0xA)]
#[machine_state(Register::SI = 0xB)]
#[machine_state(0xA + 0xB + 0xD0C = 0xAA)]
#[machine_state(0xA + 0xB + 0xD0C + 1 = 0xBB)]
fn test_mov_16bit_mem_to_reg_2byte_displacement(mut machine: Machine) {
    // MOV CX, [BX + SI + 0xD0C]
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[machine_test]
#[machine_state(0xA + 0xB + 0xD0C = 0xAA)]
fn test_mov_8bit_mem_to_reg_and_backwards_keeping_all_values_same(mut machine: Machine) {
    // MOV BX, 0xA
    // MOV SI, 0xB
    // MOV CL, [BX + SI + 0xD0C]
    // MOV CL, [BX + SI + 0xD0C]
    // MOV [BX + SI + 0xD0C], CL
    // MOV [BX + SI + 0xD0C], CL
    machine.run_instruction(Instruction::MovImm16(Register::BX, 0xA));
    machine.run_instruction(Instruction::MovImm16(Register::SI, 0xB));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CL)));
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[machine_test]
#[machine_state(0xA + 0xB + 0xD0C = 0xAA)]
#[machine_state(0xA + 0xB + 0xD0C + 1 = 0xBB)]
fn test_mov_16bit_mem_to_reg_and_backwards_keeping_all_values_same(mut machine: Machine) {
    // MOV BX, 0xA
    // MOV SI, 0xB
    // MOV CX, [BX + SI + 0xD0C]
    // MOV CX, [BX + SI + 0xD0C]
    // MOV [BX + SI + 0xD0C], CX
    // MOV [BX + SI + 0xD0C], CX
    machine.run_instruction(Instruction::MovImm16(Register::BX, 0xA));
    machine.run_instruction(Instruction::MovImm16(Register::SI, 0xB));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));
    machine.run_instruction(Instruction::Mov(Operand::Register(Register::CX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    })));
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CX)));
    machine.run_instruction(Instruction::Mov(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xD0C,
        displacement_size: 2,
    }), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[machine_test]
#[machine_state(0x01BB = 0xCC)]
fn test_mov_acc_mem_to_8bit_reg(mut machine: Machine) {
    // MOV AL, [0x01BB]
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AL), MovMemOperand::MemoryPtr(0x01BB)));

    machine.step();
    assert_eq!(machine.get_register(Register::AL), 0xCC);
}

#[machine_test]
#[machine_state(0x01BB = 0xCC)]
#[machine_state(0x01BB + 1 = 0xFF)]
fn test_mov_acc_mem_to_16bit_reg(mut machine: Machine) {
    // MOV AX, [0x01BB]
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AX), MovMemOperand::MemoryPtr(0x01BB)));

    assert_eq!(machine.get_register(Register::AX), 0xFFCC);
}

#[machine_test]
#[machine_state(Register::AL = 0xFF)]
fn test_mov_8bit_reg_to_acc_mem(mut machine: Machine) {
    // MOV [0x01BB], AL
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AL)));

    assert_eq!(machine.memory().data[0x01BB], 0xFF);
}

#[machine_test]
#[machine_state(Register::AX = 0xFFAA)]
fn test_mov_16bit_reg_to_acc_mem(mut machine: Machine) {
    // MOV [0x01BB], AX
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AX)));

    assert_eq!(machine.memory().data[0x01BB], 0xAA);
    assert_eq!(machine.memory().data[0x01BB + 1], 0xFF);
}

#[machine_test]
#[machine_state(Register::AL = 0xAA)]
fn test_mov_8bit_acc_mem_to_reg_and_backwards_keeping_all_values_same(mut machine: Machine) {
    // MOV [0x01BB], AL
    // MOV [0x01BB], AL
    // MOV AL, [0x01BB]
    // MOV AL, [0x01BB]
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AL)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AL)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AL), MovMemOperand::MemoryPtr(0x01BB)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AL), MovMemOperand::MemoryPtr(0x01BB)));

    assert_eq!(machine.get_register(Register::AL), 0xAA);
    assert_eq!(machine.memory().data[0x01BB], 0xAA);
}

#[machine_test]
#[machine_state(Register::AX = 0xFFAA)]
fn test_mov_16bit_acc_mem_to_reg_and_backwards_keeping_all_values_same(mut machine: Machine) {
    // MOV [0x01BB], AX
    // MOV [0x01BB], AX
    // MOV AX, [0x01BB]
    // MOV AX, [0x01BB]
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AX)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::MemoryPtr(0x01BB), MovMemOperand::Register(Register::AX)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AX), MovMemOperand::MemoryPtr(0x01BB)));
    machine.run_instruction(Instruction::MovAccMem(MovMemOperand::Register(Register::AX), MovMemOperand::MemoryPtr(0x01BB)));

    assert_eq!(machine.get_register(Register::AX), 0xFFAA);
    assert_eq!(machine.memory().data[0x01BB], 0xAA);
    assert_eq!(machine.memory().data[0x01BB + 1], 0xFF);
}