extern crate sdl2; 

use firefly::Game;
use game_container::game_container::{GameContainer, Keys};
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use sdl2::sys::KeyCode;
use std::time::Duration;

enum InternalEvent {
    KeyEvent { k: Keys },
    QuitEvent,
    Nothing
}

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

    'running: loop {
        // Background
        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.clear();

        // Game init
        let mut game = Game::new(800, 600);
        
        // Event parsing for basic console commands
        let events = event_pump.poll_iter().map(|ev| {
            match ev {
                Event::KeyDown { keycode: Some(Keycode::Up), .. } => {
                    InternalEvent::KeyEvent { k: Keys::Up }
                }
                Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                    InternalEvent::KeyEvent { k: Keys::Down }
                }
                Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                    InternalEvent::KeyEvent { k: Keys::Right }
                }
                Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                    InternalEvent::KeyEvent { k: Keys::Left }
                }
                Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                    InternalEvent::KeyEvent { k: Keys::B }
                }
                Event::KeyDown { keycode: Some(Keycode::Return), ..} => {
                    InternalEvent::KeyEvent { k: Keys::A }
                }
                Event::KeyDown { keycode: Some(Keycode::Tab), ..} => {
                    InternalEvent::KeyEvent { k: Keys::Select }
                }
                Event::KeyDown { keycode: Some(Keycode::Backquote), ..} => {
                    InternalEvent::KeyEvent { k: Keys::Start }
                }
                Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    InternalEvent::QuitEvent
                },
                _ => {
                    InternalEvent::Nothing
                }
            }
        });

        let mut events_for_game: Vec<Keys> = Vec::new();
        for game_event in events.into_iter() {
            match game_event {
                InternalEvent::KeyEvent { k } => { events_for_game.push(k) }
                InternalEvent::QuitEvent => { break 'running; }
                InternalEvent::Nothing => { }
            }
        }
        // Game state update
        game.tick(&events_for_game);

        // Painting
        for y in 0..game.height {
            for x in 0..game.width {
                let idx: usize = (y as usize * game.width as usize) + x as usize;
                let pixel = &game.framebuffer[idx];

                canvas.set_draw_color(Color { r: pixel.r, g: pixel.g, b: pixel.b, a: pixel.a });
                let _ = canvas.draw_point(Point::new(x as i32, y as i32));
            }
        }

        // Flushing framebuffer
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}