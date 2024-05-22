use core::panic;
use crate::{cpu::Cpu, font, screen};


#[derive(Debug)]
pub struct Chip8 {
    cpu: Cpu,
    memory: [u8; 4096],
    pub display: [i32; screen::DISPLAY_WIDTH * screen::DISPLAY_HEIGHT],
    keypad: [i32; 16],
    pub draw_flag: bool,
}

impl Chip8 {
    pub fn new() -> Chip8 {
        let mut chip8 = Chip8 {
            cpu: Cpu::new(),
            memory: [0; 4096],
            display: [0; screen::DISPLAY_WIDTH * screen::DISPLAY_HEIGHT],
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
        let opcode: u16 = u16::from(self.memory[self.cpu.pc as usize]) << 8
            | u16::from(self.memory[self.cpu.pc as usize + 1]);

        // Increment program counter here, to avoid having to do it on every
        // function for each instruction
        self.cpu.pc += 2;

        // Decode opcode
        //
        // The bitwise & creates a mask to get the nibble (4 bits) of the instruction
        // that we need for each case.
        // For example: 0xANNN & 0xF000 will yield 0xA000, so we need to shift >> 12
        // to get 0xA.
        // In the case of the first nibble, shifting right by 12 is enough
        println!(
            "Current opcode: {:#x} first nibble: {:#x} second nibble: {:#x} third nibble: {:#x} fourth nibble: {:#x}",
            opcode,
            opcode >> 12, 
            (opcode & 0x0F00) >> 8, 
            (opcode & 0x00F0) >> 4, 
            opcode & 0x000F, 
        );
        match opcode >> 12 {
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
                _ => panic!("Unknown opcode starting with 0: {:#04x}", opcode),
            },
            1 => {
                // JP addr (jump)
                // mask the last three bits to get the address to jump to
                let addr = (opcode & 0x0FFF) as u16;
                self.jump(addr);
            }
            2 => {
                // CALL addr
                let addr = (opcode & 0x0FFF) as u16;
                self.call_subroutine(addr);
            }
            3 => {
                // SE Vx, byte
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let value: u8 = (opcode & 0x00FF) as u8;
                self.skip_equal(reg, value);
            }
            4 => {
                // SNE Vx, byte
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let value: u8 = (opcode & 0x00FF) as u8;
                self.skip_not_equal(reg, value);
            }
            5 => {
                // SE Vx, Vy
                let reg1: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let reg2: u8 = ((opcode & 0x00F0) >> 4) as u8;
                self.skip_equal_registers(reg1, reg2);
            }
            6 => {
                // LD Vx, byte
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let val: u8 = (opcode & 0x00FF) as u8;
                self.load_register_vx(reg, val);
            }
            7 => {
                // ADD Vx, byte
                let reg: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let val: u8 = (opcode & 0x00FF) as u8;
                self.add_value_to_register_vx(reg, val);
            }
            8 => {
                let reg1: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let reg2: u8 = ((opcode & 0x00F0) >> 4) as u8;

                match opcode & 0x000F {
                    0 => {
                        // LD Vx, Vy
                        self.set_registers(reg1, reg2);
                    }
                    1 => {
                        // OR Vx, Vy
                        self.or_registers(reg1, reg2);
                    }
                    2 => {
                        // AND Vx, Vy
                        self.and_registers(reg1, reg2);
                    }
                    3 => {
                        // XOR Vx, Vy
                        self.xor_registers(reg1, reg2);
                    }
                    4 => {}
                    5 => {}
                    6 => {}
                    7 => {}
                    14 => {}
                    _ => panic!("Unknown operation: {:#x}", opcode),
                }
            }
            9 => {
                // SNE Vx, Vy
                let reg1: u8 = ((opcode & 0x0F00) >> 8) as u8;
                let reg2: u8 = ((opcode & 0x00F0) >> 4) as u8;
                self.skip_not_equal_registers(reg1, reg2);
            }
            10 => {
                // LD I, addr
                let val: u16 = (opcode & 0x0FFF) as u16;
                self.set_index_register(val);
            }
            11 => {}
            12 => {}
            13 => {
                // DXYN (draw srpite to the screen)
                let n = (opcode & 0x000F) as u8;
                let x = ((opcode & 0x0F00) >> 8) as u8;
                let y = ((opcode & 0x00F0) >> 4) as u8;
                self.draw_sprite_to_screen(x, y, n);
            }
            14 => {}
            15 => {}
            _ => panic!("Unknown opcode: {:#x}", opcode),
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
    }

    fn return_from_subroutine(&mut self) {
        // Decrement sp first, so it points to
        // the last element of the stack and
        // assign that element to the program counter
        self.cpu.sp -= 1;
        self.cpu.pc = self.cpu.stack.pop().expect("Illegal stack access");
    }

    fn jump(&mut self, addr: u16) {
        self.cpu.pc = addr;
    }

    fn call_subroutine(&mut self, addr: u16) {
        self.cpu.sp += 1;
        self.cpu.stack.push(self.cpu.pc);
        self.cpu.pc = addr;
    }

    fn skip_equal(&mut self, reg: u8, val: u8) {
        if self.cpu.v[reg as usize] == val {
            self.cpu.pc += 2;
        }
    }

    fn skip_not_equal(&mut self, reg: u8, val: u8) {
        if self.cpu.v[reg as usize] != val {
            self.cpu.pc += 2;
        }
    }

    fn skip_equal_registers(&mut self, reg1: u8, reg2: u8) {
        if self.cpu.v[reg1 as usize] == self.cpu.v[reg2 as usize] {
            self.cpu.pc += 2;
        }
    }

    fn skip_not_equal_registers(&mut self, reg1: u8, reg2: u8) {
        if self.cpu.v[reg1 as usize] != self.cpu.v[reg2 as usize] {
            self.cpu.pc += 2;
        }
    }

    fn load_register_vx(&mut self, reg: u8, val: u8) {
        self.cpu.v[reg as usize] = val;
    }

    fn set_registers(&mut self, reg1: u8, reg2: u8) {
        self.cpu.v[reg1 as usize] = self.cpu.v[reg2 as usize];
    }

    fn or_registers(&mut self, reg1: u8, reg2: u8) {
        self.cpu.v[reg1 as usize] |= self.cpu.v[reg2 as usize];
    }

    fn and_registers(&mut self, reg1: u8, reg2: u8) {
        self.cpu.v[reg1 as usize] &= self.cpu.v[reg2 as usize];
    }

    fn xor_registers(&mut self, reg1: u8, reg2: u8) {
        self.cpu.v[reg1 as usize] ^= self.cpu.v[reg2 as usize];
    }

    fn add_value_to_register_vx(&mut self, reg: u8, val: u8) {
        self.cpu.v[reg as usize] += val;
    }

    fn set_index_register(&mut self, val: u16) {
        self.cpu.i = val;
    }

    fn draw_sprite_to_screen(&mut self, inst_x: u8, inst_y: u8, n: u8) {
        // Mod by display width (64) or height (32) to wrap around
        let x = self.cpu.v[inst_x as usize] % screen::DISPLAY_WIDTH as u8;
        let y = self.cpu.v[inst_y as usize] % screen::DISPLAY_HEIGHT as u8;
        self.cpu.v[0x0F] = 0;

        // Loop trough each row
        for y_line in 0..n {
            let pixel = self.memory[(self.cpu.i + y_line as u16) as usize];
            // Loop through each one of the 8 bits of the row
            for x_line in 0..8 {
                // Check if the pixel value is 1
                if (pixel & (0x80 >> x_line)) != 0 {
                    // Get the index for the display array
                    // (Might not need to wrap around on this one, check later)
                    let index = ((x + x_line) as usize + ((y + y_line) as usize * 64)) % 2048;
                    // Check if the pixel is alrady active (collision)
                    if self.display[index as usize] == 1 {
                        self.cpu.v[0x0F] = 1;
                    }
                    // set the pixel value using XOR
                    self.display[index as usize] ^= 1;
                }
            }
        }

        self.draw_flag = true;
    }
}
