mod game;
mod util;

use game::Game;
use std::{thread::sleep, time::Duration};
use util::State;

use sdl2::{
    event::Event,
    keyboard::{KeyboardState, Keycode, Scancode},
};

const WINDOW_WIDTH: u32 = 800;
const WINDOW_HEIGHT: u32 = 600;

const FPS: u32 = 60;
const SPEED: f32 = 400.0;
const DT: f32 = 1.0 / FPS as f32;

fn main() {
    let sdl = sdl2::init().unwrap();
    let video = sdl.video().unwrap();

    let window = video
        .window("Breakout", WINDOW_WIDTH, WINDOW_HEIGHT)
        .position_centered()
        .build()
        .unwrap();

    let mut game = Game::new();
    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .unwrap();

    game.render(&mut canvas);

    let mut event_pump = sdl.event_pump().unwrap();

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Q),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => match game.state {
                    State::Paused => game.state = State::Playing,
                    State::Playing => game.state = State::Paused,
                },
                Event::KeyDown {
                    keycode: Some(Keycode::Space),
                    ..
                } => {
                    game.state = State::Playing;
                }
                _ => {}
            }
        }

        let keyboard = KeyboardState::new(&event_pump);

        game.bar_dir_x = 0.0;
        if keyboard.is_scancode_pressed(Scancode::A) {
            game.bar_dir_x += -1.0;
        }
        if keyboard.is_scancode_pressed(Scancode::D) {
            game.bar_dir_x += 1.0;
        }

        if game.state == State::Playing {
            game.update(DT);
            game.render(&mut canvas);
        }

        sleep(Duration::new(0, 1_000_000_000u32 / FPS));
    }
}
