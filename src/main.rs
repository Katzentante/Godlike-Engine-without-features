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

    let pos = Point::new(34, 78);

    let origin = Vec3 {
        x: 0.0,
        y: 0.0,
        // z: 6.0,
        z: 0.0,
    };
    let y_axis = Vec3 {
        x: 0.0,
        y: 10.0,
        z: 0.0,
    };
    let x_axis = Vec3 {
        x: 10.0,
        y: 0.0,
        z: 0.0,
    };
    let z_axis = Vec3 {
        x: 0.0,
        y: 0.0,
        // z: 6.0,
        z: 10.0,
    };

    let mut cam = Camera {
        fov: 100.0,
        aspect_ratio: 16.0 / 9.0,
        near: 2.0,
        far: 10.0,
        eye: Vec3 {
            x: 5.0,
            y: 5.0,
            z: 5.0,
        },
        at: Vec3 {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        // ist rechwinklig zu der ebene und zeigt in die obere Mitte
        up: Vec3 {
            x: -1.0,
            y: -1.0,
            z: 2.0,
        },
    };

    let size = window.size();
    let mut canvas = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut i = 0;
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
                Event::KeyDown {
                    keycode: Some(Keycode::D),
                    ..
                } => {
                    cam.eye.y += SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::S),
                    ..
                } => {
                    cam.eye.x -= SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::W),
                    ..
                } => {
                    cam.eye.x += SPEED;
                }
                Event::KeyDown {
                    keycode: Some(Keycode::A),
                    ..
                } => {
                    cam.eye.y -= SPEED;
                }
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        // canvas.draw_line(pos_mouse, pos).expect("Never error!");

        let pos1 = get_projected(&cam, &origin, size.0 as f32, size.1 as f32);
        let pos2 = get_projected(&cam, &y_axis, size.0 as f32, size.1 as f32);
        let pos3 = get_projected(&cam, &x_axis, size.0 as f32, size.1 as f32);
        let pos4 = get_projected(&cam, &z_axis, size.0 as f32, size.1 as f32);
        println!();

        // canvas.draw_line(pos1, pos2).unwrap();
        // canvas.draw_line(pos2, pos3).unwrap();
        // canvas.draw_line(pos3, pos4).unwrap();
        // Y
        canvas.set_draw_color(Color::GREEN);
        canvas.draw_line(pos4, pos1).unwrap();

        // X
        canvas.set_draw_color(Color::BLUE);
        canvas.draw_line(pos3, pos1).unwrap();

        // Z
        canvas.set_draw_color(Color::RED);
        canvas.draw_line(pos2, pos1).unwrap();

        canvas.set_draw_color(Color::WHITE);
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

fn get_projected(cam: &Camera, original: &Vec3, final_width: f32, final_height: f32) -> Point {
    // atan: zahl -> grad
    // tan: grad -> zahl
    // for testing now disregard z
    println!("cam pos: {:?}", cam.eye);
    let epvec = &Vec3::from_points(&cam.eye, original);
    let eavec = Vec3::from_points(&cam.eye, &cam.at);
    let alpha = eavec.cross_angle(epvec);
    // let length1 = alpha.acos() / cam.near;
    let length1 = cam.near / alpha.cos();
    let r = length1 / epvec.len();
    let epstrichvec = r * epvec;
    let mut pstrich = &cam.eye + &epstrichvec;
    // print!("p': {:?}", pstrich);
    // convert fov to radians
    let width = 2.0 * cam.near / (cam.fov * PI / 360.0).tan();
    let height = width / cam.aspect_ratio;

    let r = cam.near / eavec.len();
    let enstrichvec = r * &eavec;
    let nstrich = &cam.eye + &enstrichvec;

    let mut up = cam.up.clone();
    // for every one so that n' is at 0,0,0
    pstrich = &pstrich - &nstrich;
    let mut eye_new = &cam.eye - &nstrich;

    // println!(
    //     " p'': {:?} enew: {:?} n': {:?} up: {:?}",
    //     pstrich, eye_new, nstrich, up
    // );

    // winkel for rotation around y-axis
    let yalpha = if cam.eye.x == 0.0 {
        -std::f32::consts::FRAC_PI_2
    } else {
        (eye_new.z / eye_new.x).atan() - std::f32::consts::FRAC_PI_2
    };
    let yrot = Matrix3x3::new(
        yalpha.cos(),
        0.0,
        yalpha.sin(),
        0.0,
        1.0,
        0.0,
        -(yalpha.sin()),
        0.0,
        yalpha.cos(),
    );
    eye_new = &yrot * &eye_new;
    pstrich = &yrot * &pstrich;
    up = &yrot * &up;
    // println!(
    //     "e'': {:?} -> (after y) {:?} p''': {:?} up'': {:?}",
    //     eye_new,
    //     yalpha / PI * 180.0,
    //     pstrich,
    //     up
    // );
    // println!("{:?}", eye_new);
    //
    let xalpha = if eye_new.z == 0.0 {
        0.0
    } else {
        (eye_new.y / eye_new.z).atan()
    };
    let xrot = Matrix3x3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        xalpha.cos(),
        -(xalpha.sin()),
        0.0,
        xalpha.sin(),
        xalpha.cos(),
    );
    eye_new = &xrot * &eye_new;
    pstrich = &xrot * &pstrich;
    up = &xrot * &up;
    // println!(
    //     "e''': {:?} -> (after x) {:?} p'''': {:?} up''': {:?}",
    //     eye_new,
    //     xalpha / PI * 180.0,
    //     pstrich,
    //     up
    // );

    let zalpha = if up.x > 0.0 {
        up.cross_angle(&vec3::IDENTITY_Y)
    } else {
        -up.cross_angle(&vec3::IDENTITY_Y)
    };
    let zrot = Matrix3x3::new(
        zalpha.cos(),
        -(zalpha.sin()),
        0.0,
        zalpha.sin(),
        zalpha.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );
    eye_new = &zrot * &eye_new;
    pstrich = &zrot * &pstrich;
    up = &zrot * &up;
    // println!(
    //     "(after z) {:?} p'5: {:?} up'''': {:?}",
    //     zalpha / PI * 180.0,
    //     pstrich,
    //     up
    // );

    pstrich = Vec3 {
        x: (pstrich.x / (height / 2.0) + 1.0) / 2.0,
        y: (-pstrich.y / (width / 2.0) + 1.0) / 2.0,
        z: pstrich.z / ((cam.far - cam.near) / 2.0),
    };
    println!("{:?}", pstrich);

    // TODO NOW
    // - ROTATE AROUND Z ACHISS SO THAT EDGE OF PRJECTION SCREEN Y = 0
    // - rotate y 180 degreeas => y: -y
    // - 90 nach links auf z achse => x:
    // - ALL X / WIDTH/2
    // - ALL Y / HEIGHT/2
    // - MOVE Z TO MIDDLE OF NEAR AND FAR && ALL Z / (FAR-NEAR)/2

    // println!("{:?}", epstrichvec.clone());
    // println!("{:?}", &zrot * &eye_new);
    // println!("{:?} -> {:?}", zalpha / PI * 180.0, eye_new);
    Point::new(
        (pstrich.x * final_width) as i32,
        (pstrich.y * final_height) as i32,
    )
}
