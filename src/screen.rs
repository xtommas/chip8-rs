use crate::chip8::Chip8;
use sdl2::{pixels::Color, rect::Rect, render::Canvas, video::Window, EventPump};

pub const DISPLAY_WIDTH: usize = 64;
pub const DISPLAY_HEIGHT: usize = 32;
pub const DISPLAY_SCALE: usize = 10;

pub fn setup_screen() -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window(
            "CHIP-8",
            (DISPLAY_WIDTH * DISPLAY_SCALE).try_into().unwrap(),
            (DISPLAY_HEIGHT * DISPLAY_SCALE).try_into().unwrap(),
        )
        .position_centered()
        .build()
        .unwrap();

    let canvas = window.into_canvas().present_vsync().build().unwrap();
    let event_pump = sdl_context.event_pump().unwrap();

    return (canvas, event_pump);
}

pub fn draw_screen(chip8: &Chip8, canvas: &mut Canvas<Window>) {
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear();

    let screen_buf = chip8.display;
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    for (i, pixel) in screen_buf.iter().enumerate() {
        if *pixel != 0 {
            let x = (i % DISPLAY_WIDTH) as u32;
            let y = (i / DISPLAY_WIDTH) as u32;
            // Draw a rectangle at (x, y), scaled up by our SCALE value
            let rect = Rect::new(
                (x * DISPLAY_SCALE as u32) as i32,
                (y * DISPLAY_SCALE as u32) as i32,
                DISPLAY_SCALE.try_into().unwrap(),
                DISPLAY_SCALE.try_into().unwrap(),
            );
            canvas.fill_rect(rect).unwrap();
        }
    }
    canvas.present();
}
