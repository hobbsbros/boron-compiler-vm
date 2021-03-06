// crate::vm::memory
// Memory management for the Boron Virtual Machine

// BVM overview:
//     2^28 = 268435456 bytes (268 MB) of virtual RAM
//     2^26 = 67108864 bytes (67 MB) of program storage
//
//     16 64-bit general-purpose registers
//     1 32-bit program counter
//     

// Memory map:
//     0x00 00 00 00 to 0x03 FF FF FF -> program storage
//     0x04 00 00 00 to 0x13 FF FF FF -> virtual random access memory

// Register map:
//     0x0 to 0xE -> general-purpose registers
//     0xF        -> program counter

const MEM_SIZE: usize = 335544319;
pub const NUM_REGISTERS: usize = 16;

pub struct VirtualMachine {
    pub memory: Vec<u8>,
    pub registers: Vec<u64>,
    pub pc: u32,
    pub stack: Vec<u64>,
}

pub fn initialize() -> VirtualMachine {
    VirtualMachine {memory: vec![0u8; MEM_SIZE], registers: vec![0u64; NUM_REGISTERS], pc: 0u32, stack: Vec::new()}
}

impl VirtualMachine {
    // Loads programs from Config into virtual memory
    pub fn load_program(&mut self, program: Vec<u8>) {
        let new_program = &program;
        self.memory.splice(0..program.len(), new_program.iter().cloned());
    }

    // Gets a value from memory at a given pointer
    pub fn get(&mut self, pointer: u32) -> u8 {
        self.memory[pointer as usize]
    }

    // Loads a value into memory at a given pointer
    pub fn store(&mut self, value: u8, pointer: u32) {
        self.memory[pointer as usize] = value;
    }

    // Gets a value from memory, with the pointer given by a value in a given register
    pub fn get_register(&mut self, register: u8) -> u8 {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize]
    }

    // Loads a value into memory, with the pointer given by a value in a given register
    pub fn store_register(&mut self, value: u8, register: u8) {
        let pointer: usize = self.registers[register as usize] as usize;
        self.memory[pointer as usize] = value;
    }

    // Store a value in a given register
    pub fn to_register(&mut self, value: u64, register: u8) {
        if register < 15 {
            self.registers[register as usize] = value;
        } else if register == 15 {
            self.pc = value as u32;
        }
    }

    // Get a value from a register
    pub fn from_register(&mut self, register: u8) -> u64 {
        if register < 15 {
            self.registers[register as usize]
        } else {
            self.pc.into()
        }
    }

    // Get the value at the current program counter and increment the program counter
    pub fn next(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        self.inc(1);
        value
    }

    // Get the value at the current program counter without incrementing the program counter
    pub fn peek(&mut self) -> u8 {
        let value = self.memory[self.pc as usize];
        value
    }

    // Increment the program counter by a specific number
    pub fn inc(&mut self, increment: u32) {
        self.pc += increment;
    }

    // Set the program counter
    pub fn set_pc(&mut self, new_pc: u32) {
        self.pc = new_pc;
    }

    // Gets a slice of 4 bytes and converts it into one 32-bit value
    pub fn get_u32(&mut self) -> u32 {
        let mut slice: Vec<u8> = Vec::new();
        for _ in 0..4 {
            self.next();
        }
        for _ in 0..4 {
            let next: u8 = self.next();
            slice.push(next);
        }
        let mut result: u32 = 0;
        for value in slice {
            result = result << 4;
            result += value as u32;
        }
        result
    }
    
    // Gets a slice of 8 bytes and converts it into one 64-bit value
    pub fn get_u64(&mut self) -> u64 {
        let mut slice: Vec<u8> = Vec::new();
        for _ in 0..8 {
            let next: u8 = self.next();
            slice.push(next);
        }
        let mut result: u64 = 0;
        for value in slice {
            result = result << 8;
            result += value as u64;
        }
        result
    }

    // Pushes a given value onto the stack
    pub fn push(&mut self, value: u64) {
        self.stack.push(value);
    }

    // Pops the top value off of the stack
    pub fn pop(&mut self) -> u64 {
        if let Some(value) = self.stack.pop() {
            value
        } else {
            // TODO: Implement more robust error handling
            panic!("Tried to pop a value off of an empty stack");
        }
    }
}