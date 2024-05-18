use crate::{cpu::Cpu, font};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

pub struct Chip8 {
    cpu: Cpu,
    memory: [u8; 4096],
    display: [i32; DISPLAY_WIDTH * DISPLAY_HEIGHT],
    keypad: [i32; 16],
    draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: Cpu::new(),
            memory: [0; 4096],
            display: [0; DISPLAY_WIDTH * DISPLAY_HEIGHT],
            keypad: [0; 16],
            draw_flag: false,
        };
        chip8.load_font_set();
        chip8
    }

    // loads the fontset from 0x050 to 0x09F
    fn load_font_set(&mut self) {
        for i in 80..160 {
            self.memory[i] = font::FONT_SET[i - 80];
        }
    }
}
