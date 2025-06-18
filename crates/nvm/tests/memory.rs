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