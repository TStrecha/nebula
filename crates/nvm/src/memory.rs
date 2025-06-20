
pub const MEMORY_SIZE: usize = 16 * 1024;

pub struct LinearMemory {
    pub data: [u8; MEMORY_SIZE]
}

impl Default for LinearMemory {
    fn default() -> Self {
        Self {
            data: [0; MEMORY_SIZE]
        }
    }
}
