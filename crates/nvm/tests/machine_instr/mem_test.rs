use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::register::Register;
use nvm_test_utils::{machine_test, machine_state};

#[machine_test]
#[machine_state(Register::SP = 0xAA)]
#[machine_state(Register::AX = 0xFFBB)]
fn test_push(mut machine: Machine) {
    // PUSH AX
    machine.run_instruction(Instruction::Push(Register::AX));

    assert_eq!(machine.get_register(Register::AX), 0xFFBB);
    assert_eq!(machine.get_register(Register::SP), 0xAA - 2);
    assert_eq!(machine.memory().data[0xAA - 2], 0xBB);
    assert_eq!(machine.memory().data[0xAA - 1], 0xFF);
}

#[machine_test]
#[machine_state(Register::SP = 0xAA)]
#[machine_state(0xAA = 0xBB)]
#[machine_state(0xAA + 1 = 0xAA)]
fn test_pop(mut machine: Machine) {
    // POP AX
    machine.run_instruction(Instruction::Pop(Register::AX));

    assert_eq!(machine.get_register(Register::SP), 0xAA + 2);
    assert_eq!(machine.get_register(Register::AX), 0xAABB);
    assert_eq!(machine.memory().data[0xAA], 0xBB);
    assert_eq!(machine.memory().data[0xAA + 1], 0xAA);
}

#[machine_test]
#[machine_state(Register::SP = 0xAA)]
#[machine_state(Register::AX = 0xFFBB)]
fn test_push_pop(mut machine: Machine) {
    // PUSH AX
    // POP AX
    machine.run_instruction(Instruction::Push(Register::AX));
    machine.run_instruction(Instruction::Pop(Register::AX));

    assert_eq!(machine.get_register(Register::SP), 0xAA);
    assert_eq!(machine.get_register(Register::AX), 0xFFBB);
    assert_eq!(machine.memory().data[0xAA - 2], 0xBB);
    assert_eq!(machine.memory().data[0xAA - 1], 0xFF);
}