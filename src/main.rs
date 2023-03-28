use std::time::Duration;

use config::{Config, FileFormat};
use log::{debug, error, info};
use maths::mat3x3::Matrix3x3;
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
        pos: Vec3::new(0.0, 5.0, 5.0),
        target: maths::vec3::ZERO,
        up: maths::vec3::IDENTITY_Y,
    };

    // pyramid
    #[rustfmt::skip]
    let vertices = vec![
        crate::maths::vec3::ZERO, // A
        Vec3::new(0.0, 0.0, 2.0), // B
        Vec3::new(2.0, 0.0, 2.0), // C
        Vec3::new(2.0, 0.0, 0.0), // D
        Vec3::new(1.0, 3.0, 1.0), // S
    ];

    #[rustfmt::skip]
    let lines = vec![
        0,1,
        1,2,
        2,3,
        3,0,
        0,4,
        1,4,
        2,4,
        3,4
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
        canvas.set_draw_color(Color::BLACK);
        lines
            .iter()
            .step_by(2)
            .zip(lines.iter().skip(1).step_by(2))
            .for_each(|(start, end)| {
                // debug!("{}, {}", start, end);
                debug!("{:?}->{:?}", start, end);
                let start = get_projected(&cam, &vertices[*start], size);
                let end = get_projected(&cam, &vertices[*end], size);
                if let Err(e) = canvas.draw_line(start, end) {
                    error!("Error while drawing triangle line: {:?}", e);
                }
                debug!("");
            });
        debug!("");

        // canvas
        //     .fill_rect(Rect::new(
        //         (size.0 / 2.0) as i32,
        //         (size.1 / 2.0) as i32,
        //         20,
        //         20,
        //     ))
        //     .expect("Could not fill middle rect");
        canvas.draw_line(Point::new(size.0 as i32 / 2, 0), Point::new(size.0 as i32/ 2, size.1 as i32)).unwrap();
        canvas.draw_line(Point::new(0, size.1 as i32 / 2), Point::new(size.0 as i32, size.1 as i32 / 2)).unwrap();
        canvas.set_draw_color(Color::WHITE);

        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}

/// Returns the actual pixel position on screen of the projected original Vec3 through the given camera
fn get_projected(cam: &PerspectiveCamera, original: &Vec3, window_size: (f32, f32)) -> Point {
    // debug!("{:?}", original);
    // first impl (ortographic) just in one direction
    // tan = grad -> Verhältnis
    // tan-1 = Verhältnis -> Grad

    // n(n1, n2 n3) <- from e to a
    // o(x, y ,z) <- orthogonal zu n

    // n1*x + n2*y + n3*z = 0

    // let height_near = 2.0 * cam.near * (cam.fovy / 2.0).tan();
    // let width_near = cam.aspect_ratio * height_near;
    // let height_far = 2.0 * cam.far * (cam.fovy / 2.0).tan();
    // let width_far = cam.aspect_ratio * height_far;
    let height = 2.0 * cam.near * (cam.fovy / 2.0).tan();
    let width = cam.aspect_ratio * height;
    let depth = cam.far - cam.near;

    // setze target auf (0,0,0)

    let mut point = original - &cam.target;
    let mut cam_pos_new = &cam.pos - &cam.target;
    let mut up_pos = &cam_pos_new + &cam.up;

    // drehe um y-Achse !! x sollte nun 0 sein

    let alphay = if 0.0 == cam_pos_new.z {
        0.0
    } else {
        (cam_pos_new.x / cam_pos_new.z).atan()
    };
    let rot_y = Matrix3x3::new(
        alphay.cos(),
        0.0,
        alphay.sin(),
        0.0,
        1.0,
        0.0,
        -(alphay.sin()),
        0.0,
        alphay.cos(),
    );
    point = &rot_y * &point;
    cam_pos_new = &rot_y * &cam_pos_new;
    up_pos = &rot_y * &up_pos;

    // drehe um x-Achse so dass eye auf z-Achse

    let alphax = if 0.0 == cam_pos_new.z {
        0.0
    } else {
        (cam_pos_new.y / cam_pos_new.z).atan()
    };
    let rot_x = Matrix3x3::new(
        1.0,
        0.0,
        0.0,
        0.0,
        alphax.cos(),
        -(alphax.sin()),
        0.0,
        alphax.sin(),
        alphax.cos(),
    );
    point = &rot_x * &point;
    cam_pos_new = &rot_x * &cam_pos_new;
    up_pos = &rot_x * &up_pos;

    // drehe um z-Achse do dass up = identety-Y
    let up_after_rot = Vec3::from_points(&cam_pos_new, &up_pos);
    let alpha = up_after_rot.cross_angle(&vec3::IDENTITY_Y);
    // FIXME
    // if alpha > 180°
    // let alphaz = if alpha > std::f32::consts::PI {
    //     alpha - std::f32::consts::FRAC_PI_2
    // } else {
    //     alpha
    // };
    //
    let alphaz = if 0.0 == up_after_rot.y {
        if up_after_rot.x.is_sign_positive() {
            std::f32::consts::FRAC_PI_2
        } else {
            -std::f32::consts::FRAC_PI_2
        }
    } else {
        (up_after_rot.x / up_after_rot.y).atan()
    };
    let rot_z = Matrix3x3::new(
        alphaz.cos(),
        -(alphaz.sin()),
        0.0,
        alphaz.sin(),
        alphaz.cos(),
        0.0,
        0.0,
        0.0,
        1.0,
    );
    point = &rot_z * &point;
    cam_pos_new = &rot_z * &cam_pos_new;

    debug!(
        "xa: {}, ya: {}, za: {}",
        to_gradient(alphax),
        to_gradient(alphay),
        to_gradient(alphaz)
    );
    debug!("cam_z_Achse: {:?}", cam_pos_new);
    debug!("up windkwl y-Achse: {:?}", alpha);
    debug!("point: ({}, {})", point.x / width, point.y / height);

    // drehe up new vektor so dass er gleich wie y-Achse ist

    // später so dass at auf 0,0,0 und eye auf z achse und up=(0,1,0) und dacnn in Mittlepunkt geschoben/gedreht wird

    point = &point - &cam_pos_new;
    cam_pos_new = -&cam_pos_new;
    let near_z = cam_pos_new.z - cam.near;
    let depth_z = near_z + depth / 2.0;

    point.z += -depth_z;

    // now everything from -1 to 1

    point.x = point.x / width;
    point.y = point.y / height;
    point.z = point.z / depth;

    // now convert form [-1;1] to [0;1]

    point.x = (point.x + 1.0) / 2.0;
    point.y = (-point.y + 1.0) / 2.0;
    point.z = (point.z + 1.0) / 2.0;

    // debug!("{:?}", point);
    Point::new(
        (point.x * window_size.0) as i32,
        (point.y * window_size.1) as i32,
    )
    // (0, 0).into()
}

fn to_gradient(rad: f32) -> f32 {
    180.0 * rad / std::f32::consts::PI
}
