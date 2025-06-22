use nvm::instruction::Instruction;
use nvm::Machine;
use nvm::register::Register;
use nvm_test_utils::{machine_test, machine_state};

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_near_forward(mut machine: Machine) {
    let jumped = machine.run_instruction(Instruction::JmpNear(20));

    assert!(jumped);
    assert_eq!(machine.get_register(Register::IP), 0xFF + 20);
}


#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_near_backward(mut machine: Machine) {
    let jumped = machine.run_instruction(Instruction::JmpNear(-20));

    assert!(jumped);
    assert_eq!(machine.get_register(Register::IP), 0xFF - 20);
}

#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_far(mut machine: Machine) {
    let jumped = machine.run_instruction(Instruction::JmpFar(0, 20));

    assert!(jumped);
    assert_eq!(machine.get_register(Register::IP), 20);
}


#[machine_test]
#[machine_state(Register::IP = 0xFF)]
fn test_jmp_far_with_segment(mut machine: Machine) {
    let jumped = machine.run_instruction(Instruction::JmpFar(2, 20));

    assert!(jumped);
    assert_eq!(machine.get_register(Register::IP), 20);
    assert_eq!(machine.get_register(Register::CS), 2);
}
