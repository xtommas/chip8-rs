use chip8_rs::chip8::Chip8;
use chip8_rs::keyboard;
use chip8_rs::screen;
use core::panic;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::env;
use std::thread;
use std::time::Duration;

const TICKS_PER_FRAME: usize = 10;
const FRAME_DURATION: Duration = Duration::from_millis(16); // Targeting ~60 FPS

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
        let frame_start = std::time::Instant::now();

        // Check for Quit event
        for event in event_pump.poll_iter() {
            match event {
                // Quit with escape key
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'gameloop;
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keyboard::key_to_button(key) {
                        chip8.keypress(k, 1);
                    }
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    if let Some(k) = keyboard::key_to_button(key) {
                        chip8.keypress(k, 0);
                    }
                }
                _ => (),
            }
        }

        // Emulate one cycle
        for _ in 0..TICKS_PER_FRAME {
            chip8.emulate_cycle();
        }

        // Tick timers
        chip8.tick_timers();

        // if the instructions are 0x00E0 (clear the screen)
        // or 0xDXYN (draw sprite to the screen), update the screen
        // if chip8.draw_flag {
        //     chip8.draw_flag = false;
        //     // draw_graphics();
        //     screen::draw_screen(&chip8, &mut canvas);
        // }
        screen::draw_screen(&chip8, &mut canvas);

        // Frame rate control
        let frame_duration = frame_start.elapsed();
        if frame_duration < FRAME_DURATION {
            thread::sleep(FRAME_DURATION - frame_duration);
        }

        // Store key press state (press and realease)
        // chip8.set_keys();
    }
}
