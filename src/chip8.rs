use core::panic;

use crate::{cpu::Cpu, font};

const DISPLAY_WIDTH: usize = 64;
const DISPLAY_HEIGHT: usize = 32;

#[derive(Debug)]
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

    // Loads the fontset from 0x050 to 0x09F
    fn load_font_set(&mut self) {
        for i in 80..160 {
            self.memory[i] = font::FONT_SET[i - 80];
        }
    }

    pub fn load_rom(&mut self, file_path: &String) {
        let bytes = std::fs::read(file_path).expect("No such file or directory");
        // Programs start at memory address 0x200 (512)
        let mut i = 512;
        for byte in bytes.iter() {
            self.memory[i] = *byte;
            i += 1;
        }
    }

    pub fn emulate_cycle(&mut self) {
        // Fetch opcode
        // An instruction is 2 bytes, so we need to read two consecutive bytes
        // from memory and combine them into one 16-bit instruction
        let opcode: u16 =
            self.memory[self.cpu.pc as usize] << 8 | self.memory[self.cpu.pc as usize + 1];

        // Decode opcode
        // the bitwise & creates a mask to get the first 4 bits of the instruction
        // for example: 0xANNN & 0xF000 will yield A
        match opcode & 0xF000 {
            0 => match opcode {
                0x00E0 => {
                    // CLS
                    self.clear_display();
                }
                0x00EE => {
                    // RET
                    // return from a subrutine
                    self.return_from_subroutine();
                }
                _ => panic!("Unknown opcode: {:#04x}", opcode),
            },
            1 => {}
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {}
            7 => {}
            8 => {}
            9 => {}
            10 => {}
            11 => {}
            12 => {}
            13 => {}
            14 => {}
            15 => {}
            _ => panic!("Unknown opcode: {:#04x}", opcode),
        }

        // Update timers
        if self.cpu.delay_timer > 0 {
            self.cpu.delay_timer -= 1;
        }
    }

    fn clear_display(&mut self) {
        for pixel in self.display.iter_mut() {
            *pixel = 0;
        }
    }

    fn return_from_subroutine(&mut self) {
        // set the program counter to the address at
        // the top of the stack, and decrement the stack
        // pointer by 1
        self.cpu.pc = self.cpu.stack[self.cpu.sp as usize];
        self.cpu.sp -= 1;
    }
}
