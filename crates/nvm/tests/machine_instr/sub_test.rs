use nvm::Machine;
use nvm::instruction::Instruction;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::{Flag, Register};
use nvm_test_utils::{machine_state, machine_test};

#[machine_test]
#[machine_state(Register::AL = 0x0A)]
fn test_sub_acc_8(mut machine: Machine) {
    // SUB AL, 0x02
    machine.run_instruction(Instruction::SubAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0x0A - 0x02);
}

#[machine_test]
#[machine_state(Register::AL = 0x00)]
fn test_sub_acc_8_wrapping(mut machine: Machine) {
    // SUB AL, 0x02
    machine.run_instruction(Instruction::SubAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0xFE);
}

#[machine_test]
#[machine_state(Register::AX = 0x2211)]
fn test_sub_acc_16(mut machine: Machine) {
    // SUB AX, 0x2211
    machine.run_instruction(Instruction::SubAcc16(0x1122));

    assert_eq!(machine.get_register(Register::AX), 0x2211 - 0x1122);
}

#[machine_test]
#[machine_state(Register::AX = 0x3333)]
fn test_sub_acc_16_wrapping(mut machine: Machine) {
    // SUB AX, 0x2211
    machine.run_instruction(Instruction::SubAcc16(0x3335));

    assert_eq!(machine.get_register(Register::AX), 0xFFFE);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x55)]
fn test_sub_8bit_reg_to_mem(mut machine: Machine) {
    // SUB [BX + SI], AL
    machine.run_instruction(Instruction::Sub(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AL),
        true,
    ));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x55 - 0x22);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x02)]
#[machine_state(0x11 + 0x22 = 0x00)]
fn test_sub_8bit_reg_to_mem_wrapping(mut machine: Machine) {
    // SUB [BX + SI], AL
    machine.run_instruction(Instruction::Sub(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AL),
        true,
    ));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0xFE);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x1111)]
#[machine_state(0x11 + 0x22 = 0x22)]
#[machine_state(0x11 + 0x22 + 1 = 0x44)]
fn test_sub_16bit_reg_to_mem(mut machine: Machine) {
    // SUB [BX + SI], AX
    machine.run_instruction(Instruction::Sub(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AX),
        false,
    ));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x4422 - 0x1111);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x1113)]
#[machine_state(0x11 + 0x22 = 0x11)]
#[machine_state(0x11 + 0x22 + 1 = 0x11)]
fn test_sub_16bit_reg_to_mem_wrapping(mut machine: Machine) {
    // SUB [BX + SI], AX
    machine.run_instruction(Instruction::Sub(
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        Operand::Register(Register::AX),
        false,
    ));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0xFFFE);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_sub_mem_to_8bit_reg(mut machine: Machine) {
    // SUB AL, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AL),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x00)]
#[machine_state(0x11 + 0x22 = 0x02)]
fn test_sub_mem_to_8bit_reg_wrapping(mut machine: Machine) {
    // SUB AL, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AL),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0xFE);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_sub_mem_to_16bit_reg(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x3333)]
#[machine_state(0x11 + 0x22 = 0x35)]
#[machine_state(0x11 + 0x22 + 1 = 0x33)]
fn test_sub_mem_to_16bit_reg_wrapping(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0,
            displacement_size: 0,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0xFFFE);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x33 = 0x11)]
fn test_sub_mem_to_16bit_reg_1byte_displacement(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x33,
            displacement_size: 1,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x3333 = 0x11)]
fn test_sub_mem_to_16bit_reg_2byte_displacement(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Memory(MemAddress {
            base: Some(Register::BX),
            index: Some(Register::SI),
            displacement: 0x3333,
            displacement_size: 2,
        }),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AL = 0x22)]
fn test_sub_8bit_reg_to_8bit_reg(mut machine: Machine) {
    // SUB AL, CL
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AL),
        Operand::Register(Register::CL),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0x22 - 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x24)]
#[machine_state(Register::AL = 0x22)]
fn test_sub_8bit_reg_to_8bit_reg_wrapping(mut machine: Machine) {
    // SUB AL, CL
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AL),
        Operand::Register(Register::CL),
        true,
    ));

    assert_eq!(machine.get_register(Register::AL), 0xFE);
}

#[machine_test]
#[machine_state(Register::CX = 0x11)]
#[machine_state(Register::AX = 0x2233)]
fn test_sub_16bit_reg_to_16bit_reg(mut machine: Machine) {
    // SUB AX, CX
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::CX = 0x3335)]
#[machine_state(Register::AX = 0x3333)]
fn test_sub_16bit_reg_to_16bit_reg_wrapping(mut machine: Machine) {
    // SUB AX, CX
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
        false,
    ));

    assert_eq!(machine.get_register(Register::AX), 0xFFFE);
}

#[machine_test]
#[machine_state(Register::AX = 0x0100)]
#[machine_state(Register::CX = 0x01)]
fn test_dec_reg(mut machine: Machine) {
    // DEC AX
    // DEC CX
    machine.run_instruction(Instruction::Dec(Register::AX));
    machine.run_instruction(Instruction::Dec(Register::CX));

    assert_eq!(machine.get_register(Register::AX), 0xFF);
    assert_eq!(machine.get_register(Register::CX), 0x0000);
}

#[machine_test]
#[machine_state(Register::AX = 0x0000)]
fn test_dec_reg_wrapping(mut machine: Machine) {
    // DEC AX
    machine.run_instruction(Instruction::Dec(Register::AX));

    assert_eq!(machine.get_register(Register::AX), 0xFFFF);
}

#[machine_test]
#[machine_state(Register::AX = 0x01)]
pub fn test_sub_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Register(Register::AX),
        false,
    ));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x11)]
#[machine_state(Register::CX = 0x01)]
pub fn test_sub_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::Sub(
        Operand::Register(Register::AX),
        Operand::Register(Register::CX),
        false,
    ));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x01)]
pub fn test_sub_acc_8bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::SubAcc8(1));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x10)]
pub fn test_sub_acc_8bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::SubAcc8(1));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x01)]
pub fn test_sub_acc_16bit_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::SubAcc16(1));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x10)]
pub fn test_sub_acc_16bit_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::SubAcc16(1));

    assert!(!machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x01)]
pub fn test_dec_updating_flags_true(mut machine: Machine) {
    machine.run_instruction(Instruction::Dec(Register::AX));

    assert!(machine.get_flag(Flag::ZERO));
}

#[machine_test]
#[machine_state(Register::AX = 0x10)]
pub fn test_dec_updating_flags_false(mut machine: Machine) {
    machine.run_instruction(Instruction::Dec(Register::AX));

    assert!(!machine.get_flag(Flag::ZERO));
}
