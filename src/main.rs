use chip8_rs::chip8::Chip8;
use chip8_rs::screen;
use core::panic;
use sdl2::event::Event;
use std::env;

fn main() {
    // setupGraphics()
    let result = screen::setup_screen();
    let mut canvas = result.0;
    let mut event_pump = result.1;
    canvas.clear();
    canvas.present();

    // setupInput()

    let mut chip8 = Chip8::new();
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        panic!("Provide the path to the rom to run as the first argument");
    }
    let rom = &args[1];
    chip8.load_rom(rom);

    //Emulation loop
    'gameloop: loop {
        // Check for Quit event
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'gameloop;
                }
                _ => (),
            }
        }

        // Emulate one cycle
        chip8.emulate_cycle();

        // if the instructions are 0x00E0 (clear the screen)
        // or 0xDXYN (draw sprite to the screen), update the screen
        if chip8.draw_flag {
            chip8.draw_flag = false;
            // draw_graphics();
            screen::draw_screen(&chip8, &mut canvas);
        }

        // Store key press state (press and realease)
        // chip8.set_keys();
    }
}
