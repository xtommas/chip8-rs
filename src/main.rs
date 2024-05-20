use chip8_rs::chip8::Chip8;
use std::env;

fn main() {
    // setupGraphics()
    // setupInput()

    let mut chip8 = Chip8::new();
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];
    chip8.load_rom(rom);

    //Emulation loop
    loop {
        // Emulate one cycle
        chip8.emulate_cycle();

        // if the instructions are 0x00E0 (clear the screen)
        // or 0xDXYN (draw sprite to the screen), update the screen
        if chip8.draw_flag {
            // draw_graphics();
        }

        // Store key press state (press and realease)
        // chip8.set_keys();
    }
}
