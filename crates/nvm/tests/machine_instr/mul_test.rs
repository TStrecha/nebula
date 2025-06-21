use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;
use nvm_test_utils::{machine_state, machine_test};

#[machine_test]
#[machine_state(Register::AL = 0x05)]
#[machine_state(Register::CL = 0x04)]
fn test_mul_with_8bit_reg(mut machine: Machine) {
    // MUL BYTE CL
    machine.run_instruction(Instruction::Mul8(Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x05 * 0x04);
}

#[machine_test]
#[machine_state(Register::AL = 0xAA)]
#[machine_state(Register::CL = 0xBB)]
fn test_mul_with_8bit_reg_overflow(mut machine: Machine) {
    // MUL BYTE CL
    machine.run_instruction(Instruction::Mul8(Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x2E);
    assert_eq!(machine.get_register(Register::AH), 0x7C);
}

#[machine_test]
#[machine_state(Register::AL = 0x02)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0x0A)]
fn test_mul_8_with_mem(mut machine: Machine) {
    // MUL BYTE [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Mul8(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AL), 0x14);
}

#[machine_test]
#[machine_state(Register::AL = 0xAA)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0x0B)]
fn test_mul_8_with_mem_overflow(mut machine: Machine) {
    // MUL BYTE [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Mul8(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AL), 0x4E);
    assert_eq!(machine.get_register(Register::AH), 0x07);
}

#[machine_test]
#[machine_state(Register::AX = 0x0AAA)]
#[machine_state(Register::CX = 0x02)]
fn test_mul_with_16bit_reg(mut machine: Machine) {
    // MUL WORD CX
    machine.run_instruction(Instruction::Mul16(Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), 0x0AAA * 0x02);
}

#[machine_test]
#[machine_state(Register::AX = 0xAAAA)]
#[machine_state(Register::CX = 0x0B)]
fn test_mul_with_16bit_reg_overflow(mut machine: Machine) {
    // MUL WORD CL
    machine.run_instruction(Instruction::Mul16(Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), 0x554e);
    assert_eq!(machine.get_register(Register::DX), 0x0007);
}

#[machine_test]
#[machine_state(Register::AX = 0x02)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0xAA)]
#[machine_state(0x11 + 0x22 + 0xFF + 1 = 0x0A)]
fn test_mul_16_with_mem(mut machine: Machine) {
    // MUL WORD [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Mul16(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AX), 0x0AAA * 0x02);
}

#[machine_test]
#[machine_state(Register::AX = 0xAAAA)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0x0B)]
fn test_mul_16_with_mem_overflow(mut machine: Machine) {
    // MUL WORD [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Mul16(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AX), 0x554e);
    assert_eq!(machine.get_register(Register::DX), 0x0007);
}
