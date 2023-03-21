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
use std::time::Duration;

mod camera;
mod maths;

use maths::vector::Vec3;

use crate::camera::camera::Camera;

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

const SPEED: i32 = 5;

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

    let mut pos = Point::new(34, 78);
    let mut pos_mouse = Point::new(300, 300);

    let eck1 = Vec3 {
        x: 0.0,
        y: 3.0,
        // z: 6.0,
        z: 0.0,
    };
    let eck2 = Vec3 {
        x: 0.0,
        y: 6.0,
        z: 0.0,
    };
    let eck3 = Vec3 {
        x: 4.0,
        y: 0.0,
        z: 0.0,
    };
    let eck4 = Vec3 {
        x: 4.0,
        y: 0.0,
        // z: 6.0,
        z: 0.0,
    };

    let cam = Camera {
        fov: 90.0,
        aspect_ratio: 16.0 / 9.0,
        near: 2.0,
        far: 10.0,
        eye: Vec3 {
            x: 7.0,
            y: 7.0,
            z: 0.0,
        },
        at: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        up: crate::maths::vector::IDENTITY_Y3,
    };


    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 64, 255));
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
    'running: loop {
        i = (i + 1) % 255;
        canvas.set_draw_color(Color::RGB(i, 64, 255 - i));
        canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    pos.x += SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    pos.y += SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    pos.y -= SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    pos.x -= SPEED;
                }
                Event::MouseMotion { x, y, .. } => pos_mouse = (x, y).into(),
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        canvas.set_draw_color(Color::BLACK);
        canvas.draw_line(pos_mouse, pos).expect("Never error!");

        let pos1 = get_projected(&cam, &eck1);
        let pos2 = get_projected(&cam, &eck2);
        let pos3 = get_projected(&cam, &eck3);
        let pos4 = get_projected(&cam, &eck4);

        canvas.draw_line(pos1, pos2).unwrap();
        canvas.draw_line(pos2, pos3).unwrap();
        canvas.draw_line(pos3, pos4).unwrap();
        canvas.draw_line(pos4, pos1).unwrap();

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_projected(cam: &Camera, original: &Vec3) -> Point {
    // for testing now disregard z
    // let distx = (cam.eye.x - original.x).abs();
    // let disty = (cam.eye.x - original.x).abs();
    let alpha = Vec3::from_points(&cam.eye, &cam.at).cross_angle(&Vec3::from_points(&cam.eye, original));
    let length1 = alpha.acos() / cam.near;
    // println!("{}", );
    Point::new(cam.near as i32 * 30, length1 as i32 * 30)
}
