use nvm::register::Register;

#[test]
fn test_8bit_register_from_code() {
    assert_eq!(Register::from_register_code(0, true), Ok(Register::AL));
    assert_eq!(Register::from_register_code(1, true), Ok(Register::CL));
    assert_eq!(Register::from_register_code(2, true), Ok(Register::DL));
    assert_eq!(Register::from_register_code(3, true), Ok(Register::BL));
    assert_eq!(Register::from_register_code(4, true), Ok(Register::AH));
    assert_eq!(Register::from_register_code(5, true), Ok(Register::CH));
    assert_eq!(Register::from_register_code(6, true), Ok(Register::DH));
    assert_eq!(Register::from_register_code(7, true), Ok(Register::BH));
}

#[test]
fn test_16bit_register_from_code() {
    assert_eq!(Register::from_register_code(0, false), Ok(Register::AX));
    assert_eq!(Register::from_register_code(1, false), Ok(Register::CX));
    assert_eq!(Register::from_register_code(2, false), Ok(Register::DX));
    assert_eq!(Register::from_register_code(3, false), Ok(Register::BX));
    assert_eq!(Register::from_register_code(4, false), Ok(Register::SP));
    assert_eq!(Register::from_register_code(5, false), Ok(Register::BP));
    assert_eq!(Register::from_register_code(6, false), Ok(Register::SI));
    assert_eq!(Register::from_register_code(7, false), Ok(Register::DI));
}

#[test]
fn test_8bit_register_from_invalid_code() {
    assert!(Register::from_register_code(0x0F, true).is_err());
}

#[test]
fn test_16bit_register_from_invalid_code() {
    assert!(Register::from_register_code(0xFF, false).is_err());
}
