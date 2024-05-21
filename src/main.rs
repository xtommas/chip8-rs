use chip8_rs::chip8::Chip8;
use core::panic;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window};
use std::env;

fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = chip8.display;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel != 0 {
            let x = (i % 64) as u32;
            let y = (i / 64) as u32;

            // Draw a rectangle scaled up by 10, in our case
            let rect = Rect::new((x * 10) as i32, (y * 10) as i32, 10, 10);
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}

fn main() {
    // setupGraphics()
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("The game", 640, 320)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas = window.into_canvas().present_vsync().build().unwrap();
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
    loop {
        // Emulate one cycle
        chip8.emulate_cycle();

        // if the instructions are 0x00E0 (clear the screen)
        // or 0xDXYN (draw sprite to the screen), update the screen
        if chip8.draw_flag {
            chip8.draw_flag = false;
            // draw_graphics();
            draw_screen(&chip8, &mut canvas);
        }

        // Store key press state (press and realease)
        // chip8.set_keys();
    }
}
