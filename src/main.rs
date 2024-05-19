use chip8_rs::chip8::Chip8;
use std::env;

fn main() {
    // setupGraphics()
    // setupInput()

    let mut chip8 = Chip8::new();
    let args: Vec<String> = env::args().collect();
    let rom = &args[1];
    println!("memory before: {:?}", chip8.memory);
    chip8.load_rom(rom);
    println!("memory after: {:?}", chip8.memory);
    //Emulation loop
    // loop {
    //      Emulate one cycle
    //      chip8.emulate_cycle();
    //
    //      if the instructions are 0x00E0 or 0xDXYN, update the screen
    //      draw_graphics();
    //
    //      // Store key press state (press and realease)
    //      chip8.set_keys();
    // }
    //
}
