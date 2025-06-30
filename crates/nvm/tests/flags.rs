use nvm::Machine;
use nvm::register::Flag;
use nvm_test_utils::machine_test;

#[test]
pub fn test_set_flag_carry() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::CARRY, true);
    assert!(machine.get_flag(Flag::CARRY));
    machine.set_flag(Flag::CARRY, false);
    assert!(!machine.get_flag(Flag::CARRY))
}

#[test]
pub fn test_set_flag_parity() {
    let mut machine = Machine::default();
    machine.set_flag(Flag::PARITY, true);
    assert!(machine.get_flag(Flag::PARITY));

    machine.set_flag(Flag::PARITY, false);
    assert!(!machine.get_flag(Flag::PARITY))
}

#[test]
pub fn test_set_flag_auxiliary() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::AUXILIARY, true);
    assert!(machine.get_flag(Flag::AUXILIARY));

    machine.set_flag(Flag::AUXILIARY, false);
    assert!(!machine.get_flag(Flag::AUXILIARY))
}

#[test]
pub fn test_set_flag_zero() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::ZERO, true);
    assert!(machine.get_flag(Flag::ZERO));

    machine.set_flag(Flag::ZERO, false);
    assert!(!machine.get_flag(Flag::ZERO))
}

#[test]
pub fn test_set_flag_sign() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::SIGN, true);
    assert!(machine.get_flag(Flag::SIGN));

    machine.set_flag(Flag::SIGN, false);
    assert!(!machine.get_flag(Flag::SIGN))
}

#[test]
pub fn test_set_flag_trap() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::TRAP, true);
    assert!(machine.get_flag(Flag::TRAP));

    machine.set_flag(Flag::TRAP, false);
    assert!(!machine.get_flag(Flag::TRAP))
}

#[test]
pub fn test_set_flag_interrupt() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::INTERRUPT, true);
    assert!(machine.get_flag(Flag::INTERRUPT));

    machine.set_flag(Flag::INTERRUPT, false);
    assert!(!machine.get_flag(Flag::INTERRUPT))
}

#[test]
pub fn test_set_flag_direction() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::DIRECTION, true);
    assert!(machine.get_flag(Flag::DIRECTION));

    machine.set_flag(Flag::DIRECTION, false);
    assert!(!machine.get_flag(Flag::DIRECTION))
}

#[test]
pub fn test_set_flag_overflow() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::OVERFLOW, true);
    assert!(machine.get_flag(Flag::OVERFLOW));

    machine.set_flag(Flag::OVERFLOW, false);
    assert!(!machine.get_flag(Flag::OVERFLOW))
}

#[test]
pub fn test_set_flag_all() {
    let mut machine = Machine::default();

    machine.set_flag(Flag::CARRY, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);

    machine.set_flag(Flag::PARITY, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);

    machine.set_flag(Flag::AUXILIARY, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);

    machine.set_flag(Flag::ZERO, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);

    machine.set_flag(Flag::SIGN, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);
    assert_eq!(machine.get_flag(Flag::SIGN), true);

    machine.set_flag(Flag::TRAP, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);
    assert_eq!(machine.get_flag(Flag::SIGN), true);
    assert_eq!(machine.get_flag(Flag::TRAP), true);

    machine.set_flag(Flag::INTERRUPT, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);
    assert_eq!(machine.get_flag(Flag::SIGN), true);
    assert_eq!(machine.get_flag(Flag::TRAP), true);
    assert_eq!(machine.get_flag(Flag::INTERRUPT), true);

    machine.set_flag(Flag::DIRECTION, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);
    assert_eq!(machine.get_flag(Flag::SIGN), true);
    assert_eq!(machine.get_flag(Flag::TRAP), true);
    assert_eq!(machine.get_flag(Flag::INTERRUPT), true);
    assert_eq!(machine.get_flag(Flag::DIRECTION), true);

    machine.set_flag(Flag::OVERFLOW, true);
    assert_eq!(machine.get_flag(Flag::CARRY), true);
    assert_eq!(machine.get_flag(Flag::PARITY), true);
    assert_eq!(machine.get_flag(Flag::AUXILIARY), true);
    assert_eq!(machine.get_flag(Flag::ZERO), true);
    assert_eq!(machine.get_flag(Flag::SIGN), true);
    assert_eq!(machine.get_flag(Flag::TRAP), true);
    assert_eq!(machine.get_flag(Flag::INTERRUPT), true);
    assert_eq!(machine.get_flag(Flag::DIRECTION), true);
    assert_eq!(machine.get_flag(Flag::OVERFLOW), true);
}

#[machine_test]
pub fn test_update_zero_flag(mut machine: Machine) {
    machine.update_zero_flag(1);
    assert!(!machine.get_flag(Flag::ZERO));

    machine.update_zero_flag(0);
    assert!(machine.get_flag(Flag::ZERO));
}
