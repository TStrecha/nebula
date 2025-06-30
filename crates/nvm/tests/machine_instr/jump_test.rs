use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::register::Register;
use nvm::register::Flag;
use nvm_test_utils::{machine_test, machine_state};

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_near_forward(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpNear(20));

    assert_eq!(machine.get_register(Register::IP), 0xFF + 20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_near_backward(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpNear(-20));

    assert_eq!(machine.get_register(Register::IP), 0xFF - 20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_short_near_forward(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpShort(20));

    assert_eq!(machine.get_register(Register::IP), 0xFF + 20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_short_near_backward(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpShort(-20));

    assert_eq!(machine.get_register(Register::IP), 0xFF - 20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_far(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpFar(0, 20));

    assert_eq!(machine.get_register(Register::IP), 20);
}


#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_far_with_segment(mut machine: Machine) {
    machine.run_instruction(Instruction::JmpFar(2, 20));

    assert_eq!(machine.get_register(Register::IP), 20);
    assert_eq!(machine.get_register(Register::CS), 2);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
#[machine_state(Flag::ZERO = false)]
fn test_jz_fail(mut machine: Machine) {
    machine.run_instruction(Instruction::Jz(0x20));

    assert_eq!(machine.get_register(Register::IP), 0xFF);
}


#[machine_test]
#[machine_state(Register::IP = 0xFF)]
#[machine_state(Flag::ZERO = true)]
fn test_jz(mut machine: Machine) {
    machine.run_instruction(Instruction::Jz(0x20));

    assert_eq!(machine.get_register(Register::IP), 0xFF + 0x20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
#[machine_state(Flag::ZERO = true)]
fn test_jnz_fail(mut machine: Machine) {
    machine.run_instruction(Instruction::Jnz(0x20));

    assert_eq!(machine.get_register(Register::IP), 0xFF);
}


#[machine_test]
#[machine_state(Register::IP = 0xFF)]
#[machine_state(Flag::ZERO = false)]
fn test_jnz(mut machine: Machine) {
    machine.run_instruction(Instruction::Jnz(0x20));

    assert_eq!(machine.get_register(Register::IP), 0xFF + 0x20);
}
