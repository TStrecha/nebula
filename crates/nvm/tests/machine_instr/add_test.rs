use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;
use nvm_test_utils::{machine_test, machine_state};

#[machine_test]
#[machine_state(Register::AL = 0x0A)]
fn test_add_acc_8(mut machine: Machine) {
    // ADD AL, 0x02
    machine.run_instruction(Instruction::AddAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0x0A + 0x02);
}

#[machine_test]
#[machine_state(Register::AX = 0x1122)]
fn test_add_acc_16(mut machine: Machine) {
    // ADD AX, 0x2211
    machine.run_instruction(Instruction::AddAcc16(0x2211));

    assert_eq!(machine.get_register(Register::AX), 0x1122 + 0x2211);
}

#[machine_test]
#[machine_state(Register::AL = 0xFF)]
fn test_add_acc_8_wrapping(mut machine: Machine) {
    // ADD AL, 0x02
    machine.run_instruction(Instruction::AddAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0x01);
}

#[machine_test]
#[machine_state(Register::AX = 0xFFFF)]
fn test_add_acc_16_wrapping(mut machine: Machine) {
    // ADD AX, 0x2211
    machine.run_instruction(Instruction::AddAcc16(0x2));

    assert_eq!(machine.get_register(Register::AX), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_add_8bit_reg_to_mem(mut machine: Machine) {
    // ADD [BX + SI], AL
    machine.run_instruction(Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AL)));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x02)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_8bit_reg_to_mem_wrapping(mut machine: Machine) {
    // ADD [BX + SI], AL
    machine.run_instruction(Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AL)));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_16bit_reg_to_mem(mut machine: Machine) {
    // ADD [BX + SI], AX
    machine.run_instruction(Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AX)));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x2233 + 0xFF);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0xFFFF)]
#[machine_state(0x11 + 0x22 = 0x02)]
fn test_add_16bit_reg_to_mem_wrapping(mut machine: Machine) {
    // ADD [BX + SI], AX
    machine.run_instruction(Instruction::Add(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AX)));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_add_mem_to_8bit_reg(mut machine: Machine) {
    // ADD AL, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AL), 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x02)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_mem_to_8bit_reg_wrapping(mut machine: Machine) {
    // ADD AL, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AL), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_add_mem_to_16bit_reg(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0xFFFF)]
#[machine_state(0x11 + 0x22 = 0x02)]
fn test_add_mem_to_16bit_reg_wrapping(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AX), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x33 = 0x11)]
fn test_add_mem_to_16bit_reg_1byte_displacement(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x33,
        displacement_size: 1
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x3333 = 0x11)]
fn test_add_mem_to_16bit_reg_2byte_displacement(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x3333,
        displacement_size: 2
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AL = 0x22)]
fn test_add_8bit_reg_to_8bit_reg(mut machine: Machine) {
    // ADD AL, CL
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AL), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0xFF)]
#[machine_state(Register::AL = 0x02)]
fn test_add_8bit_reg_to_8bit_reg_wrapping(mut machine: Machine) {
    // ADD AL, CL
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AL), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x01);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AX = 0x2233)]
fn test_add_16bit_reg_to_16bit_reg(mut machine: Machine) {
    // ADD AX, CX
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}
#[machine_test]
#[machine_state(Register::CL = 0x02)]
#[machine_state(Register::AX = 0xFFFF)]
fn test_add_16bit_reg_to_16bit_reg_wrapping(mut machine: Machine) {
    // ADD AX, CX
    machine.run_instruction(Instruction::Add(Operand::Register(Register::AX), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), 0x01);
}
