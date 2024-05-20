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
            1 => {
                // JP addr (jump)
                // mask the last three bits to get the address to jump to
                let addr = (opcode & 0x0FFF) as u16;
                self.jump(addr);
            }
            2 => {}
            3 => {}
            4 => {}
            5 => {}
            6 => {
                // LD Vx, byte
                let reg: u8 = (opcode & 0x0F00) as u8;
                let value: u8 = (opcode & 0x00FF) as u8;
                self.load_register_vx(reg, value);
            }
            7 => {
                let reg: u8 = (opcode & 0x0F00) as u8;
                let value: u8 = (opcode & 0x00FF) as u8;
                self.add_value_to_register_vx(reg, value);
            }
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
        self.draw_flag = true;
        self.cpu.pc += 2;
    }

    fn return_from_subroutine(&mut self) {
        // Decrement sp first, so it points to
        // the last element of the stack and
        // assign that element to the program counter
        self.cpu.sp -= 1;
        self.cpu.pc = self.cpu.stack.pop().expect("Illegal stack access");
        self.cpu.pc += 2;
    }

    fn jump(&mut self, addr: u16) {
        self.cpu.pc = addr;
    }

    fn load_register_vx(&mut self, reg: u8, val: u8) {
        self.cpu.v[reg as usize] = val;
        self.cpu.pc += 2;
    }

    fn add_value_to_register_vx(&mut self, reg: u8, val: u8) {
        self.cpu.v[reg as usize] += val;
        self.cpu.pc += 2;
    }
}
