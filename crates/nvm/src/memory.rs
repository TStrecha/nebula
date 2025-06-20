
pub const MEMORY_SIZE: usize = 16 * 1024;

pub struct LinearMemory {
    pub data: [u8; MEMORY_SIZE]
}


impl LinearMemory {

    pub fn read_byte(&self, ptr: usize) -> u8 {
        self.data[ptr]
    }

    pub fn read_word(&self, ptr: usize) -> u16 {
        ((self.data[ptr + 1] as u16) << 8) | self.data[ptr] as u16
    }

    pub fn write_byte(&mut self, ptr: usize, value: u8) {
        self.data[ptr] = value;
    }

    pub fn write_word(&mut self, ptr: usize, value: u16) {
        self.data[ptr + 1] = (value >> 8) as u8;
        self.data[ptr] = (value & 0xFF) as u8;
    }
}

impl Default for LinearMemory {
    fn default() -> Self {
        Self {
            data: [0; MEMORY_SIZE]
        }
    }
}
