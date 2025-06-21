use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::modrm::{MemAddress, Operand};
use nvm::register::Register;
use nvm_test_utils::{machine_state, machine_test};

#[machine_test]
#[machine_state(Register::AL = 0x05)]
#[machine_state(Register::CL = 0x04)]
fn test_div_with_8bit_reg(mut machine: Machine) {
    // DIV BYTE CL
    machine.run_instruction(Instruction::Div8(Operand::Register(Register::CL)));

    assert_eq!(machine.get_register(Register::AL), 0x05 / 0x04);
    assert_eq!(machine.get_register(Register::AH), 0x05 % 0x04);
}

#[machine_test]
#[machine_state(Register::AL = 0x02)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0x0A)]
fn test_div_8_with_mem(mut machine: Machine) {
    // DIV BYTE [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Div8(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AL), 0x02 / 0x0A);
    assert_eq!(machine.get_register(Register::AH), 0x02 % 0x0A);
}

#[machine_test]
#[machine_state(Register::AX = 0xAAAA)]
#[machine_state(Register::DX = 0x00BC)]
#[machine_state(Register::CX = 0x10)]
fn test_div_with_16bit_reg(mut machine: Machine) {
    // DIV WORD CX
    machine.run_instruction(Instruction::Div16(Operand::Register(Register::CX)));

    assert_eq!(machine.get_register(Register::AX), (0xBCAAAA / 0x10_u32) as u16);
    assert_eq!(machine.get_register(Register::DX), (0xBCAAAA % 0x10_u32) as u16);
}

#[machine_test]
#[machine_state(Register::AX = 0x00AA)]
#[machine_state(Register::DX = 0x02)]
#[machine_state(Register::BX = 0x11)]
#[machine_state(Register::SI = 0x22)]
#[machine_state(0x11 + 0x22 + 0xFF = 0xAA)]
#[machine_state(0x11 + 0x22 + 0xFF + 1 = 0x0A)]
fn test_div_16_with_mem(mut machine: Machine) {
    // DIV WORD [BX + SI + 0xFF]
    machine.run_instruction(Instruction::Div16(Operand::Memory(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xFF,
        displacement_size: 1,
    })));

    assert_eq!(machine.get_register(Register::AX), (0x0200AA / 0x0AAA) as u16);
    assert_eq!(machine.get_register(Register::DX), (0x0200AA % 0x0AAA) as u16);
}
