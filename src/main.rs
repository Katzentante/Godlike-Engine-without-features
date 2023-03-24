use config::{Config, FileFormat};
use log::{debug, error, info};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use sdl2::sys::{SDL_GL_GetDrawableSize, SDL_GetWindowSize};
use sdl2::video::FullscreenType;
use serde::{Deserialize, Serialize};
use std::f32::consts::PI;
use std::time::Duration;

mod camera;
mod maths;

use maths::vec3::Vec3;

use crate::camera::camera::Camera;
use crate::maths::mat3x3::Matrix3x3;
use crate::maths::vec3;

// use log::{debug, error, log_enabled, info, Level};

// debug!("this is a debug {}", "message");
// error!("this is printed by default");

// if log_enabled!(Level::Info) {
//     let x = 3 * 4; // expensive computation
//     info!("the answer was: {}", x);
// }

#[derive(Debug, Deserialize)]
struct GameConfig {
    fullscreen: bool,
}

const SPEED: f32 = 0.5;

pub fn main() {
    env_logger::init();
    let config = Config::builder()
        .add_source(config::File::with_name("Config.toml"))
        .build()
        // .unwrap();
        .unwrap_or(
            Config::builder()
                .add_source(config::File::from_str(
                    include_str!("DefaultConfig.toml"),
                    FileFormat::Toml,
                ))
                .build()
                .unwrap(),
        );
    let config = match config.try_deserialize::<GameConfig>() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{:?}", e);
            std::process::exit(1);
        }
    };

    info!("{:?}", config);

    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let mut window = video_subsystem
        .window("Best game", 900, 600)
        .position_centered()
        .build()
        .unwrap();

    if config.fullscreen {
        if let Err(e) = window.set_fullscreen(FullscreenType::Desktop) {
            error!("{:?}", e);
        }
    }

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        // i = (i + 1) % 255;
        // canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_projected(cam: &Camera, original: &Vec3, final_width: f32, final_height: f32) -> Point {
    (0,0).into()
}
