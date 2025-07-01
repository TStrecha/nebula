use nvm::Machine;
use nvm::instruction::Instruction;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::{Flag, Register};
use nvm_test_utils::{machine_state, machine_test};

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
    machine.run_instruction(Instruction::Add(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AL),
        true,
    ));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x02)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_8bit_reg_to_mem_wrapping(mut machine: Machine) {
    // ADD [BX + SI], AL
    machine.run_instruction(Instruction::Add(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AL),
        true,
    ));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_16bit_reg_to_mem(mut machine: Machine) {
    // ADD [BX + SI], AX
    machine.run_instruction(Instruction::Add(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AX),
        false,
    ));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x2233 + 0xFF);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0xFFFF)]
#[machine_state(0x11 + 0x22 = 0x02)]
fn test_add_16bit_reg_to_mem_wrapping(mut machine: Machine) {
    // ADD [BX + SI], AX
    machine.run_instruction(Instruction::Add(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AX),
        false,
    ));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_add_mem_to_8bit_reg(mut machine: Machine) {
    // ADD AL, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AL),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x02)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_add_mem_to_8bit_reg_wrapping(mut machine: Machine) {
    // ADD AL, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AL),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_add_mem_to_16bit_reg(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0xFFFF)]
#[machine_state(0x11 + 0x22 = 0x02)]
fn test_add_mem_to_16bit_reg_wrapping(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x01);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x33 = 0x11)]
fn test_add_mem_to_16bit_reg_1byte_displacement(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x33,
            displacement_size: 1,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x3333 = 0x11)]
fn test_add_mem_to_16bit_reg_2byte_displacement(mut machine: Machine) {
    // ADD AX, [BX + SI]
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x3333,
            displacement_size: 2,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AL = 0x22)]
fn test_add_8bit_reg_to_8bit_reg(mut machine: Machine) {
    // ADD AL, CL
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AL),
        Operand::Register(Register::CL),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 + 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0xFF)]
#[machine_state(Register::AL = 0x02)]
fn test_add_8bit_reg_to_8bit_reg_wrapping(mut machine: Machine) {
    // ADD AL, CL
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AL),
        Operand::Register(Register::CL),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x01);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AX = 0x2233)]
fn test_add_16bit_reg_to_16bit_reg(mut machine: Machine) {
    // ADD AX, CX
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 + 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x02)]
#[machine_state(Register::AX = 0xFFFF)]
fn test_add_16bit_reg_to_16bit_reg_wrapping(mut machine: Machine) {
    // ADD AX, CX
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x01);
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
fn test_inc_reg(mut machine: Machine) {
    // INC AX
    // INC CX
    machine.run_instruction(Instruction::Inc(Register::AX));
    machine.run_instruction(Instruction::Inc(Register::CX));

    assert_eq!(machine.get_register(Register::AX), 0x0100);
    assert_eq!(machine.get_register(Register::CX), 0x01);
}

#[machine_test]
#[machine_state(Register::AX = 0xFFFF)]
fn test_inc_reg_wrapping(mut machine: Machine) {
    // INC AX
    machine.run_instruction(Instruction::Inc(Register::AX));

    assert_eq!(machine.get_register(Register::AX), 0x0000);
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_add_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Register(Register::AX),
        false,
    ));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x01)]
pub fn test_add_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::Add(
        Operand::Register(Register::AX),
        Operand::Register(Register::AX),
        false,
    ));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_add_acc_8bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::AddAcc8(0));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_add_acc_8bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::AddAcc8(1));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_add_acc_16bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::AddAcc16(0));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_add_acc_16bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::AddAcc16(1));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFFFF)]
pub fn test_inc_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::Inc(Register::AX));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x00)]
pub fn test_inc_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::Inc(Register::AX));

    assert!(!machine.get_flag(Flag::ZERO));
}
