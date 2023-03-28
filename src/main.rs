use std::time::Duration;

use config::{Config, FileFormat};
use log::{debug, error, info};
use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use sdl2::video::FullscreenType;
use serde::Deserialize;

mod camera;
mod maths;

use maths::vec3::Vec3;

use crate::camera::camera::PerspectiveCamera;

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
    // std::env::set_var("RUST_LOG", "error,warn,info,debug,trace");
    std::env::set_var("RUST_LOG", "info,debug");
    // std::env::set_var("RUST_LOG", "debug");
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
        .resizable()
        .build()
        .unwrap();

    if config.fullscreen {
        if let Err(e) = window.set_fullscreen(FullscreenType::Desktop) {
            error!("{:?}", e);
        }
    }

    let window_size = window.drawable_size();
    let mut cam = PerspectiveCamera {
        fovy: 90.0,
        aspect_ratio: window_size.0 as f32 / window_size.1 as f32,
        near: 2.0,
        far: 10.0,
        pos: Vec3::new(0.0, 0.0, 5.0),
        target: maths::vec3::ZERO,
        up: maths::vec3::IDENTITY_Y,
    };

    #[rustfmt::skip]
    let vertices = vec![
        // crate::maths::vec3::IDENTITY_X,
        // crate::maths::vec3::IDENTITY_Y,
        // crate::maths::vec3::IDENTITY_Z,
        Vec3::new(1.0, 1.0, 2.0),
        Vec3::new(1.0, 3.0, 3.0),
        Vec3::new(0.0, 4.0, 1.0),
        Vec3::new(1.0, 3.0, 0.0),
        crate::maths::vec3::ZERO,
    ];

    #[rustfmt::skip]
    let lines = vec![
        0, 1,
        1, 2,
        2, 0,
        3, 0,
        3, 1,
        3, 2
    ];

    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
    let mut size = match canvas.output_size() {
        Ok((x, y)) => (x as f32, y as f32),
        Err(e) => {
            error!("Could not get canvas output size: {:?}", e);
            panic!("Could not get Canvas window size: FIXME no panic");
        }
    };

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
                Event::Window {
                    win_event: WindowEvent::Resized(x, y),
                    ..
                } => {
                    cam.aspect_ratio = x as f32 / y as f32;
                    size = (x as f32, y as f32);
                }
                _ => {}
            }
        }

        // iterate over a pair of two elemtnts at a time
        lines
            .iter()
            .step_by(2)
            .zip(lines.iter().skip(1).step_by(2))
            .for_each(|(start, end)| {
                // debug!("{}, {}", start, end);
                let start = get_projected(&cam, &vertices[*start], size);
                let end = get_projected(&cam, &vertices[*end], size);
                if let Err(e) = canvas.draw_line(start, end) {
                    error!("Error while drawing triangle line: {:?}", e);
                }
            });
        // debug!("");

        canvas.set_draw_color(Color::BLACK);
        canvas.fill_rect(Rect::new((size.0 / 2.0) as i32, (size.1 / 2.0) as i32, 40, 40)).expect("Could not fill middle rect");
        canvas.set_draw_color(Color::WHITE);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/// Returns the actual pixel position on screen of the projected original Vec3 through the given camera
fn get_projected(cam: &PerspectiveCamera, original: &Vec3, window_size: (f32, f32)) -> Point {
    // debug!("{:?}", original);


    (0, 0).into()
}
