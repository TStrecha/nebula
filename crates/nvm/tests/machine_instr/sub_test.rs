use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;
use nvm_test_utils::{machine_test, machine_state};

#[machine_test]
#[machine_state(Register::AL = 0x0A)]
fn test_sub_acc_8(mut machine: Machine) {
    // SUB AL, 0x02
    machine.run_instruction(Instruction::SubAcc8(0x02));

    assert_eq!(machine.get_register(Register::AL), 0x0A - 0x02);
}

#[machine_test]
#[machine_state(Register::AX = 0x2211)]
fn test_sub_acc_16(mut machine: Machine) {
    // SUB AX, 0x2211
    machine.run_instruction(Instruction::SubAcc16(0x1122));

    assert_eq!(machine.get_register(Register::AX), 0x2211 - 0x1122);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x55)]
fn test_sub_8bit_reg_to_mem(mut machine: Machine) {
    // SUB [BX + SI], AL
    machine.run_instruction(Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AL)));

    assert_eq!(machine.memory().data[0x11 + 0x22], 0x55 - 0x22);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x1111)]
#[machine_state(0x11 + 0x22 = 0x22)]
#[machine_state(0x11 + 0x22 + 1 = 0x44)]
fn test_sub_16bit_reg_to_mem(mut machine: Machine) {
    // SUB [BX + SI], AX
    machine.run_instruction(Instruction::Sub(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    }), Operand::Register(Register::AX)));

    assert_eq!(machine.memory().read_word(0x11 + 0x22), 0x4422 - 0x1111);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AL = 0x22)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_sub_mem_to_8bit_reg(mut machine: Machine) {
    // SUB AL, [BX + SI]
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AL), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AL), 0x22 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 = 0x11)]
fn test_sub_mem_to_16bit_reg(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x33 = 0x11)]
fn test_sub_mem_to_16bit_reg_1byte_displacement(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x33,
        displacement_size: 1
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(Register::AX = 0x2233)]
#[machine_state(0x11 + 0x22 + 0x3333 = 0x11)]
fn test_sub_mem_to_16bit_reg_2byte_displacement(mut machine: Machine) {
    // SUB AX, [BX + SI]
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AX), Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0x3333,
        displacement_size: 2
    })));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AL = 0x22)]
fn test_sub_8bit_reg_to_8bit_reg(mut machine: Machine) {
    // SUB AL, CL
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AL), Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x22 - 0x11);
}

#[machine_test]
#[machine_state(Register::CL = 0x11)]
#[machine_state(Register::AX = 0x2233)]
fn test_sub_16bit_reg_to_16bit_reg(mut machine: Machine) {
    // SUB AX, CX
    machine.run_instruction(Instruction::Sub(Operand::Register(Register::AX), Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), 0x2233 - 0x11);
}
