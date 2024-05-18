pub struct Cpu {
    // 16 8-bit registers, from V0 to VF
    v: [u8; 16],
    // 16-bit index register
    i: u16,
    // 16-bit program counter
    pc: u16,
    // 8-bit stack pointer
    sp: u8,
    // Stack that can hold 16 16-bit values
    stack: Vec<u16>,
    // 8-bit delay timer
    delay_timer: u8,
    // 8-bit sound timer
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            v: [0; 16],
            i: 0,
            sp: 0,
            stack: Vec::new(),
            sound_timer: 0,
            delay_timer: 0,
            // programs start at 0x200,
            // 0x000 to 0x199 is reserver for the interpreter
            pc: 0x200,
        }
    }
}
