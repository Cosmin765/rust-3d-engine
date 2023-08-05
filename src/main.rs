extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod vectors;
mod matrices;
mod drawable;

use vectors::Vector3;
use matrices::Matrix3;
use drawable::Drawable;


fn get_cube() -> Drawable {
    let vertices = vec![
        Vector3::new(-1.0, -1.0, 1.0),
        Vector3::new(-1.0, 1.0, 1.0),
        Vector3::new(1.0, -1.0, 1.0),
        Vector3::new(1.0, 1.0, 1.0),

        Vector3::new(-1.0, -1.0, -1.0),
        Vector3::new(-1.0, 1.0, -1.0),
        Vector3::new(1.0, -1.0, -1.0),
        Vector3::new(1.0, 1.0, -1.0),
    ];

    let indices = vec![
        (0, 1), (1, 3), (3, 2), (2, 0),
        (4, 5), (5, 7), (7, 6), (6, 4),
        (0, 4), (1, 5), (2, 6), (3, 7),
    ];
    
    Drawable::new(vertices, indices)
}

fn get_recursive_structure() -> Drawable {
    let space = 200.0;

    let variations: Vec<Vector3> = vec![
        Vector3::new(1.0, 0.0, 0.0),
        Vector3::new(-1.0, 0.0, 0.0),
        Vector3::new(0.0, 1.0, 0.0),
        Vector3::new(0.0, -1.0, 0.0),
        Vector3::new(0.0, 0.0, 1.0),
        Vector3::new(0.0, 0.0, -1.0),
    ].iter().map(|v| v * space).collect();


    let mut origin_cube = get_cube();

    for variation in variations.iter() {
        let mut child = get_cube();
        child.set_origin(variation.clone());
        origin_cube.add_child(child);
    }

    origin_cube
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("This is something", 0, 0)
        .position_centered()
        .fullscreen_desktop()
        .build()
        .unwrap();

    let (width, height) = window.size();

    let mut canvas = window.into_canvas().build().unwrap();

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    
    let mut angle = 0.0;

    let mut recursive_structure = get_recursive_structure();

    let origin = Vector3::new(width as f64 / 2.0, height as f64 / 2.0, 0.0);

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                                        break 'running
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(0, 255, 255));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(255, 0, 0));

        let rotation = &Matrix3::y_rotation(angle) * &Matrix3::x_rotation(angle);
        recursive_structure.set_origin(origin.clone()).set_rotation(rotation.clone());
        recursive_structure.draw(&mut canvas, 50.0);
        
        canvas.present();

        angle += 0.05f64;
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
}
