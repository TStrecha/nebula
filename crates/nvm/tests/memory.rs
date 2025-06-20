use nvm::memory::LinearMemory;
use nvm::memory::MEMORY_SIZE;

#[test]
fn test_linear_memory_default() {
    let memory = LinearMemory::default();

    assert_eq!(memory.data.len(), MEMORY_SIZE);

    for i in 0..MEMORY_SIZE {
        assert_eq!(memory.data[i], 0);
    }
}

#[test]
fn test_read_byte() {
    let mut memory = LinearMemory::default();
    memory.data[10] = 0xFF;

    assert_eq!(memory.read_byte(10), 0xFF);
}

#[test]
fn test_read_word() {
    let mut memory = LinearMemory::default();
    memory.data[10] = 0xAA;
    memory.data[11] = 0xBB;

    assert_eq!(memory.read_word(10), 0xBBAA);
}

#[test]
fn test_write_byte() {
    let mut memory = LinearMemory::default();
    memory.write_byte(10, 0xFF);

    assert_eq!(memory.data[10], 0xFF);
}

#[test]
fn test_write_word() {
    let mut memory = LinearMemory::default();
    memory.write_word(10, 0xAABB);

    assert_eq!(memory.data[10], 0xBB);
    assert_eq!(memory.data[11], 0xAA);
}

#[test]
fn test_write_byte_read_byte() {
    let mut memory = LinearMemory::default();
    memory.write_byte(10, 0xFF);

    assert_eq!(memory.data[10], 0xFF);
    assert_eq!(memory.read_byte(10), 0xFF);
}

#[test]
fn test_write_word_read_word() {
    let mut memory = LinearMemory::default();
    memory.write_word(10, 0xAABB);

    assert_eq!(memory.data[10], 0xBB);
    assert_eq!(memory.data[11], 0xAA);
    assert_eq!(memory.read_word(10), 0xAABB);
}