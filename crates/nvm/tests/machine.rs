use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use nvm::instruction::{Opcode};
use nvm::Machine;
use nvm::modrm::{MemAddress};
use nvm::register::Register;
use nvm_test_utils::{machine_test};

mod machine_instr;

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

#[machine_test]
fn test_step_with_dynamic_instruction_size(mut machine: Machine) {
    // MOV CX, [BX + SI]
    // MOV CX, [BX + SI + 0x0C]
    // MOV CX, [BX + SI + 0xD0C]
    machine.load_program_bytes(&[
        0x8B, 0b00001000,
        0x8B, 0b01001000, 0x0C,
        0x8B, 0b10001000, 0x0C, 0xD]);

    machine.step();
    assert_eq!(machine.get_register(Register::IP), 2);

    machine.step();
    assert_eq!(machine.get_register(Register::IP), 5);

    machine.step();
    assert_eq!(machine.get_register(Register::IP), 9);
}

#[test]
fn test_mem_get_set() {
    let mut machine = Machine::default();

    machine.memory_mut().data[0] = 0x01;
    assert_eq!(machine.memory().data[0], 0x01);
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
fn test_get_ptr_from_mem_address() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xAA);
    machine.set_register(Register::SI, 0xBB);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: None,
        index: None,
        displacement: 0,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: None,
        index: None,
        displacement: 0xCC,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xCC);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: Some(Register::BX),
        index: None,
        displacement: 0,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xAA);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: None,
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xBB);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xAA + 0xBB);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: Some(Register::BX),
        index: Some(Register::SI),
        displacement: 0xCC,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xAA + 0xBB + 0xCC);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: Some(Register::BX),
        index: None,
        displacement: 0xCC,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xAA + 0xCC);

    let ptr = machine.get_ptr_from_mem_address(MemAddress {
        base: None,
        index: Some(Register::SI),
        displacement: 0xCC,
        displacement_size: 0,
    });
    assert_eq!(ptr, 0xBB + 0xCC);
}

#[test]
fn test_dump_self() {
    let mut machine = Machine::default();
    machine.set_register(Register::AX, 0xAA);
    machine.set_register(Register::CX, 0xAB);
    machine.set_register(Register::DX, 0xAC);
    machine.set_register(Register::BX, 0xAD);

    machine.dump_self();
}

#[test]
fn test_noop_instruction() {
    let mut machine = Machine::default();
    machine.load_program_bytes(&[Opcode::NOOP as u8]);
    machine.step();

    assert_eq!(machine.get_register(Register::IP), 1);
}

#[test]
fn test_jmp_not_increasing_ip() {
    let mut machine = Machine::default();
    machine.load_program_bytes(&[Opcode::JMP as u8, 0xFF, 0xFF]);
    machine.step();

    assert_eq!(machine.get_register(Register::IP), 0xFFFF);
}
