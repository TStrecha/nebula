use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use nvm::instruction::Opcode;
use nvm::Machine;
use nvm::register::Register;

#[test]
fn test_machine_default() {
    let machine = Machine::default();

    assert_eq!(machine.get_register(Register::IP), 0);
    assert!(machine.get_register(Register::SP) > 100);
}

#[test]
fn test_load_program_from_buffer() {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/resources/program.nvm");

    let file = File::open(path).expect("File not found");
    let program = BufReader::new(file);
    let buffer_len = program.get_ref().metadata().unwrap().len() as usize;

    let mut machine = Machine::default();
    machine.load_program(program);

    for x in 0..machine.memory().data.len() {
        if x < buffer_len {
            assert_ne!(machine.memory().data[x], 0x00);
        } else {
            assert_eq!(machine.memory().data[x], 0x00);
        }
    }
}

#[test]
fn test_load_byte_program() {
    let mut machine = Machine::default();
    machine.load_program_bytes(&[Opcode::NOOP as u8, 0xB4, 0x01]);

    assert_eq!(machine.memory().data[0], Opcode::NOOP as u8);
    assert_eq!(machine.memory().data[1], 0xB4);
    assert_eq!(machine.memory().data[2], 0x01);
    for x in 3..machine.memory().data.len() {
        assert_eq!(machine.memory().data[x], 0x00);
    }
}

#[test]
fn test_step() {
    let mut machine = Machine::default();
    machine.load_program_bytes(&[Opcode::NOOP as u8]);

    machine.step();

    assert_eq!(machine.get_register(Register::IP), 1);
}

#[test]
fn test_set_16bit_register() {
    let mut machine = Machine::default();

    machine.set_register(Register::AX, 0xAABB);
    assert_eq!(machine.get_register(Register::AX), 0xAABB);

    machine.set_register(Register::CX, 0xAABB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);

    machine.set_register(Register::DX, 0xAABB);
    assert_eq!(machine.get_register(Register::DX), 0xAABB);

    machine.set_register(Register::BX, 0xAABB);
    assert_eq!(machine.get_register(Register::BX), 0xAABB);

    machine.set_register(Register::SP, 0xAABB);
    assert_eq!(machine.get_register(Register::SP), 0xAABB);

    machine.set_register(Register::BP, 0xAABB);
    assert_eq!(machine.get_register(Register::BP), 0xAABB);

    machine.set_register(Register::SI, 0xAABB);
    assert_eq!(machine.get_register(Register::SI), 0xAABB);

    machine.set_register(Register::DI, 0xAABB);
    assert_eq!(machine.get_register(Register::DI), 0xAABB);
}

#[test]
fn test_set_8bit_lower_register() {
    let mut machine = Machine::default();

    machine.set_register(Register::AL, 0xAABB);
    assert_eq!(machine.get_register(Register::AL), 0xBB);
    assert_eq!(machine.get_register(Register::AX), 0xBB);

    machine.set_register(Register::CL, 0xAABB);
    assert_eq!(machine.get_register(Register::CL), 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBB);

    machine.set_register(Register::DL, 0xAABB);
    assert_eq!(machine.get_register(Register::DL), 0xBB);
    assert_eq!(machine.get_register(Register::DX), 0xBB);

    machine.set_register(Register::BL, 0xAABB);
    assert_eq!(machine.get_register(Register::BL), 0xBB);
    assert_eq!(machine.get_register(Register::BX), 0xBB);
}

#[test]
fn test_set_8bit_higher_register() {
    let mut machine = Machine::default();

    machine.set_register(Register::AH, 0xAABB);
    assert_eq!(machine.get_register(Register::AH), 0xBB);
    assert_eq!(machine.get_register(Register::AX), 0xBB00);

    machine.set_register(Register::CH, 0xAABB);
    assert_eq!(machine.get_register(Register::CH), 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBB00);

    machine.set_register(Register::DH, 0xAABB);
    assert_eq!(machine.get_register(Register::DH), 0xBB);
    assert_eq!(machine.get_register(Register::DX), 0xBB00);

    machine.set_register(Register::BH, 0xAABB);
    assert_eq!(machine.get_register(Register::BH), 0xBB);
    assert_eq!(machine.get_register(Register::BX), 0xBB00);
}

#[test]
fn test_set_16bit_using_8bit_registers() {
    let mut machine = Machine::default();

    machine.set_register(Register::AH, 0xAA);
    machine.set_register(Register::AL, 0xBB);
    assert_eq!(machine.get_register(Register::AH), 0xAA);
    assert_eq!(machine.get_register(Register::AL), 0xBB);
    assert_eq!(machine.get_register(Register::AX), 0xAABB);

    machine.set_register(Register::CH, 0xAA);
    machine.set_register(Register::CL, 0xBB);
    assert_eq!(machine.get_register(Register::CH), 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);

    machine.set_register(Register::DH, 0xAA);
    machine.set_register(Register::DL, 0xBB);
    assert_eq!(machine.get_register(Register::DH), 0xAA);
    assert_eq!(machine.get_register(Register::DL), 0xBB);
    assert_eq!(machine.get_register(Register::DX), 0xAABB);

    machine.set_register(Register::BH, 0xAA);
    machine.set_register(Register::BL, 0xBB);
    assert_eq!(machine.get_register(Register::BH), 0xAA);
    assert_eq!(machine.get_register(Register::BL), 0xBB);
    assert_eq!(machine.get_register(Register::BX), 0xAABB);
}

#[test]
fn test_noop_instruction() {
    let mut machine = Machine::default();
    machine.load_program_bytes(&[Opcode::NOOP as u8]);
    machine.step();

    assert_eq!(machine.get_register(Register::IP), 1);
}

#[test]
fn test_mov_8bit() {
    let mut machine = Machine::default();

    // MOV AL, 0xFF
    // MOV AH, 0x10
    machine.load_program_bytes(&[0xB0, 0xFF, 0xB4, 0x10]);
    machine.step();
    machine.step();

    assert_eq!(machine.get_register(Register::IP), 4);
    assert_eq!(machine.get_register(Register::AL), 0xFF);
    assert_eq!(machine.get_register(Register::AH), 0x10);

    assert_eq!(machine.get_register(Register::AX), 0x10FF);
}

#[test]
fn test_mov_16bit() {
    let mut machine = Machine::default();

    // MOV AX, 0xFF10
    machine.load_program_bytes(&[0xB8, 0x10, 0xFF]);
    machine.step();

    assert_eq!(machine.get_register(Register::IP), 3);
    assert_eq!(machine.get_register(Register::AX), 0xFF10);
}
