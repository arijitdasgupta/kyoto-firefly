extern crate sdl2; 

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::sys::KeyCode;
use std::time::Duration;
 
pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
 
    let window = video_subsystem.window("testbench", 800, 600)
        .position_centered()
        .build()
        .unwrap();
 
    let mut canvas = window.into_canvas().build().unwrap();
 
    canvas.set_draw_color(Color::RGB(0, 0, 0));
    // canvas.set_blend_mode(sdl2::render::BlendMode::None);
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;

    let mut offset_x = 0;
    let mut offset_y = 0;
    'running: loop {
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        i = (i + 1) % 255;

        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        let rect = Rect::new(offset_x, offset_y, 100, 100);

        let _ = canvas.fill_rect(rect);
        let _ = canvas.draw_rect(rect);

        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } |
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    offset_y = offset_y - 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    offset_y = offset_y + 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    offset_x = offset_x + 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    offset_x = offset_x - 1;
                }
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}