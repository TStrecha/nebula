use nvm::Machine;
use nvm::instruction::Instruction;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::{Flag, Register};
use nvm_test_utils::{machine_state, machine_test};

#[machine_test]
#[machine_state(Register::AL = 0x0A)]
fn test_and_acc_8(mut machine: Machine) {
    // AND AL, 0x02
    machine.run_instruction(Instruction::AndAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0x0A & 0x02);
}

#[machine_test]
#[machine_state(Register::AX = 0x1122)]
fn test_and_acc_16(mut machine: Machine) {
    // AND AX, 0x2211
    machine.run_instruction(Instruction::AndAcc16(0x2211));

    assert_eq!(machine.get_register(Register::AX), 0x1122 & 0x2211);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_and_8bit_reg_to_mem(mut machine: Machine) {
    // AND [BX + SI], AL
    machine.run_instruction(Instruction::And(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AL),
    ));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x22 & 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0xFF)]
fn test_and_16bit_reg_to_mem(mut machine: Machine) {
    // AND [BX + SI], AX
    machine.run_instruction(Instruction::And(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AX),
    ));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x2233 & 0xFF);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_and_mem_to_8bit_reg(mut machine: Machine) {
    // AND AL, [BX + SI]
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AL),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 & 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_and_mem_to_16bit_reg(mut machine: Machine) {
    // AND AX, [BX + SI]
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 & 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x33 = 0x11)]
fn test_and_mem_to_16bit_reg_1byte_displacement(mut machine: Machine) {
    // AND AX, [BX + SI]
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x33,
            displacement_size: 1,
        }),
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 & 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x3333 = 0x11)]
fn test_and_mem_to_16bit_reg_2byte_displacement(mut machine: Machine) {
    // AND AX, [BX + SI]
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x3333,
            displacement_size: 2,
        }),
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 & 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AL = 0x22)]
fn test_and_8bit_reg_to_8bit_reg(mut machine: Machine) {
    // AND AL, CL
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AL),
        Operand::Register(Register::CL),
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 & 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AX = 0x2233)]
fn test_and_16bit_reg_to_16bit_reg(mut machine: Machine) {
    // AND AX, CX
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 & 0x11);
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
#[machine_state(Register::CX = 0x00)]
pub fn test_and_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
    ));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
#[machine_state(Register::CX = 0x01)]
pub fn test_and_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::And(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
    ));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
pub fn test_and_acc_8bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::AndAcc8(0x00));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
pub fn test_and_acc_8bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::AndAcc8(0x01));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
pub fn test_and_acc_16bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::AndAcc16(0x00));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0xFF)]
pub fn test_and_acc_16bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::AndAcc16(0x01));

    assert!(!machine.get_flag(Flag::ZERO));
}
