use std::fs::File;
use std::io::{BufReader};
use std::path::PathBuf;
use nvm::instruction::{MemAddress, Opcode};
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
fn test_mov_reg_to_reg() {
    let mut machine = Machine::default();
    machine.set_register(Register::CL, 0xAA);
    machine.set_register(Register::DL, 0xBB);
    machine.set_register(Register::CX, 0xAAAA);
    machine.set_register(Register::DX, 0xBBBB);

    // MOV BL, CL
    // MOV AL, DL
    // MOV BX, CX
    // MOV AX, DX
    machine.load_program_bytes(&[
        0x88, 0xCB,
        0x8A, 0xC2,
        0x89, 0xCB,
        0x8B, 0xC2]);

    // 8 bit
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.get_register(Register::BL), 0xAA);
    assert_eq!(machine.get_register(Register::DL), 0xBB);
    assert_eq!(machine.get_register(Register::AL), 0xBB);

    // 16 bit
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::CX), 0xAAAA);
    assert_eq!(machine.get_register(Register::BX), 0xAAAA);
    assert_eq!(machine.get_register(Register::DX), 0xBBBB);
    assert_eq!(machine.get_register(Register::AX), 0xBBBB);

    assert_eq!(machine.get_register(Register::IP), 8);
}

#[test]
fn test_mov_8bit_reg_to_mem() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CL, 0xAA);

    // MOV [BX + SI], CL
    machine.load_program_bytes(&[
        0x88, 0b00001000]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
}

#[test]
fn test_mov_16bit_reg_to_mem() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CX, 0xAABB);

    // MOV [BX + SI], CX
    machine.load_program_bytes(&[
        0x89, 0b00001000]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 1], 0xAA);
}

#[test]
fn test_mov_8bit_reg_to_mem_with_1byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CL, 0xAA);

    // MOV [BX + SI + 0xC], CL
    machine.load_program_bytes(&[
        0x88, 0b01001000, 0xC]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
}

#[test]
fn test_mov_16bit_reg_to_mem_with_1byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CX, 0xAABB);

    // MOV [BX + SI + 0xC], CX
    machine.load_program_bytes(&[
        0x89, 0b01001000, 0xC]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC + 1], 0xAA);
}

#[test]
fn test_mov_8bit_reg_to_mem_with_2byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CL, 0xAA);

    // MOV [BX + SI + 0xD0C], CL
    machine.load_program_bytes(&[
        0x88, 0b10001000, 0x0C, 0xD]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
}

#[test]
fn test_mov_16bit_reg_to_mem_with_2byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);
    machine.set_register(Register::CX, 0xAABB);

    // MOV [BX + SI + 0xD0C], CX
    machine.load_program_bytes(&[
        0x89, 0b10001000, 0x0C, 0xD]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.get_register(Register::CX), 0xAABB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xBB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xAA);
}

#[test]
fn test_mov_8bit_mem_to_reg() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB] = 0xAA;

    // MOV CL, [BX + SI]
    machine.load_program_bytes(&[
        0x8A, 0b00001000]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[test]
fn test_mov_16bit_mem_to_reg() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB] = 0xAA;
    machine.memory_mut().data[0xA + 0xB + 1] = 0xBB;

    // MOV CX, [BX + SI]
    machine.load_program_bytes(&[
        0x8B, 0b00001000]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[test]
fn test_mov_8bit_mem_to_reg_with_1byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB + 0xC] = 0xAA;

    // MOV CL, [BX + SI + 0xC]
    machine.load_program_bytes(&[
        0x8A, 0b01001000, 0xC]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[test]
fn test_mov_16bit_mem_to_reg_1byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB + 0xC] = 0xAA;
    machine.memory_mut().data[0xA + 0xB + 0xC + 1] = 0xBB;

    // MOV CX, [BX + SI + 0xC]
    machine.load_program_bytes(&[
        0x8B, 0b01001000, 0xC]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xC + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[test]
fn test_mov_8bit_mem_to_reg_with_2byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB + 0xD0C] = 0xAA;

    // MOV CL, [BX + SI + 0xD0C]
    machine.load_program_bytes(&[
        0x8A, 0b10001000, 0x0C, 0xD]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[test]
fn test_mov_16bit_mem_to_reg_2byte_displacement() {
    let mut machine = Machine::default();
    machine.set_register(Register::BX, 0xA);
    machine.set_register(Register::SI, 0xB);

    machine.memory_mut().data[0xA + 0xB + 0xD0C] = 0xAA;
    machine.memory_mut().data[0xA + 0xB + 0xD0C + 1] = 0xBB;

    // MOV CX, [BX + SI + 0xD0C]
    machine.load_program_bytes(&[
        0x8B, 0b10001000, 0x0C, 0xD]);

    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[test]
fn test_mov_8bit_mem_to_reg_and_backwards_keeping_all_values_same() {
    let mut machine = Machine::default();
    machine.memory_mut().data[0xA + 0xB + 0xD0C] = 0xAA;

    // MOV BX, 0xA
    // MOV SI, 0xB
    // MOV CL, [BX + SI + 0xD0C]
    // MOV CL, [BX + SI + 0xD0C]
    // MOV [BX + SI + 0xD0C], CL
    // MOV [BX + SI + 0xD0C], CL
    machine.load_program_bytes(&[
        0xBB, 0x0A, 0x00,
        0xBE, 0x0B, 0x00,
        0x8A, 0b10001000, 0x0C, 0xD,
        0x8A, 0b10001000, 0x0C, 0xD,
        0x88, 0b10001000, 0x0C, 0xD,
        0x88, 0b10001000, 0x0C, 0xD,
    ]);

    machine.step();
    machine.step();
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.get_register(Register::CL), 0xAA);
}

#[test]
fn test_mov_16bit_mem_to_reg_and_backwards_keeping_all_values_same() {
    let mut machine = Machine::default();
    machine.memory_mut().data[0xA + 0xB + 0xD0C] = 0xAA;
    machine.memory_mut().data[0xA + 0xB + 0xD0C + 1] = 0xBB;

    // MOV BX, 0xA
    // MOV SI, 0xB
    // MOV CX, [BX + SI + 0xD0C]
    // MOV CX, [BX + SI + 0xD0C]
    // MOV [BX + SI + 0xD0C], CX
    // MOV [BX + SI + 0xD0C], CX
    machine.load_program_bytes(&[
        0xBB, 0x0A, 0x00,
        0xBE, 0x0B, 0x00,
        0x8B, 0b10001000, 0x0C, 0xD,
        0x8B, 0b10001000, 0x0C, 0xD,
        0x89, 0b10001000, 0x0C, 0xD,
        0x89, 0b10001000, 0x0C, 0xD,
    ]);

    machine.step();
    machine.step();
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::BX), 0xA);
    assert_eq!(machine.get_register(Register::SI), 0xB);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C], 0xAA);
    assert_eq!(machine.memory().data[0xA + 0xB + 0xD0C + 1], 0xBB);
    assert_eq!(machine.get_register(Register::CX), 0xBBAA);
}

#[test]
fn test_mov_acc_mem_to_8bit_reg() {
    let mut machine = Machine::default();
    machine.memory_mut().data[0x01BB] = 0xCC;

    // MOV AL, [0x01BB]
    machine.load_program_bytes(&[
        0xA0, 0xBB, 0x01]);

    machine.step();
    assert_eq!(machine.get_register(Register::AL), 0xCC);
}

#[test]
fn test_mov_acc_mem_to_16bit_reg() {
    let mut machine = Machine::default();
    machine.memory_mut().data[0x01BB] = 0xCC;
    machine.memory_mut().data[0x01BB + 1] = 0xFF;

    // MOV AX, [0x01BB]
    machine.load_program_bytes(&[
        0xA1, 0xBB, 0x01]);

    machine.step();
    assert_eq!(machine.get_register(Register::AX), 0xFFCC);
}

#[test]
fn test_mov_8bit_reg_to_acc_mem() {
    let mut machine = Machine::default();
    machine.set_register(Register::AL, 0xFF);

    // MOV [0x01BB], AL
    machine.load_program_bytes(&[
        0xA2, 0xBB, 0x01]);

    machine.step();
    assert_eq!(machine.memory().data[0x01BB], 0xFF);
}

#[test]
fn test_mov_16bit_reg_to_acc_mem() {
    let mut machine = Machine::default();
    machine.set_register(Register::AX, 0xFFAA);

    // MOV [0x01BB], AX
    machine.load_program_bytes(&[
        0xA3, 0xBB, 0x01]);

    machine.step();
    assert_eq!(machine.memory().data[0x01BB], 0xAA);
    assert_eq!(machine.memory().data[0x01BB + 1], 0xFF);
}

#[test]
fn test_mov_8bit_acc_mem_to_reg_and_backwards_keeping_all_values_same() {
    let mut machine = Machine::default();
    machine.set_register(Register::AL, 0xAA);

    // MOV [0x01BB], AL
    // MOV [0x01BB], AL
    // MOV AL, [0x01BB]
    // MOV AL, [0x01BB]
    machine.load_program_bytes(&[
        0xA2, 0xBB, 0x01,
        0xA2, 0xBB, 0x01,
        0xA0, 0xBB, 0x01,
        0xA0, 0xBB, 0x01
    ]);

    machine.step();
    machine.step();
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::AL), 0xAA);
    assert_eq!(machine.memory().data[0x01BB], 0xAA);
}

#[test]
fn test_mov_16bit_acc_mem_to_reg_and_backwards_keeping_all_values_same() {
    let mut machine = Machine::default();
    machine.set_register(Register::AX, 0xFFAA);

    // MOV [0x01BB], AX
    // MOV [0x01BB], AX
    // MOV AX, [0x01BB]
    // MOV AX, [0x01BB]
    machine.load_program_bytes(&[
        0xA3, 0xBB, 0x01,
        0xA3, 0xBB, 0x01,
        0xA1, 0xBB, 0x01,
        0xA1, 0xBB, 0x01
    ]);

    machine.step();
    machine.step();
    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::AX), 0xFFAA);
    assert_eq!(machine.memory().data[0x01BB], 0xAA);
    assert_eq!(machine.memory().data[0x01BB + 1], 0xFF);
}

#[test]
fn test_push() {
    let mut machine = Machine::default();
    machine.set_register(Register::SP, 0xAA);
    machine.set_register(Register::AX, 0xFFBB);

    // PUSH AX
    machine.load_program_bytes(&[0x50]);

    machine.step();
    assert_eq!(machine.get_register(Register::AX), 0xFFBB);
    assert_eq!(machine.get_register(Register::SP), 0xAA - 2);
    assert_eq!(machine.memory().data[0xAA - 2], 0xBB);
    assert_eq!(machine.memory().data[0xAA - 1], 0xFF);
}

#[test]
fn test_pop() {
    let mut machine = Machine::default();
    machine.set_register(Register::SP, 0xAA);

    machine.memory_mut().data[0xAA] = 0xBB;
    machine.memory_mut().data[0xAA + 1] = 0xAA;

    // POP AX
    machine.load_program_bytes(&[0x58]);

    machine.step();
    assert_eq!(machine.get_register(Register::SP), 0xAA + 2);
    assert_eq!(machine.get_register(Register::AX), 0xAABB);
    assert_eq!(machine.memory().data[0xAA], 0xBB);
    assert_eq!(machine.memory().data[0xAA + 1], 0xAA);
}

#[test]
fn test_push_pop() {
    let mut machine = Machine::default();
    machine.set_register(Register::SP, 0xAA);
    machine.set_register(Register::AX, 0xFFBB);

    // PUSH AX
    // POP AX
    machine.load_program_bytes(&[0x50, 0x58]);

    machine.step();
    machine.step();
    assert_eq!(machine.get_register(Register::SP), 0xAA);
    assert_eq!(machine.get_register(Register::AX), 0xFFBB);
    assert_eq!(machine.memory().data[0xAA - 2], 0xBB);
    assert_eq!(machine.memory().data[0xAA - 1], 0xFF);
}
