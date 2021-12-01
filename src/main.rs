extern crate sdl2;

use std::path::Path;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem
        .window("Productivity Simulator", 800, 600)
        .resizable()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window
        .into_canvas()
        .accelerated()
        .present_vsync()
        .build()
        .map_err(|e| e.to_string())?;

    let texture_creator = canvas.texture_creator();

    let font = ttf_context.load_font(Path::new("OpenSans-Regular.ttf"), 128)?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    let mut timer = sdl_context.timer()?;

    let mut start: u64 = 0;
    let mut end: u64 = 0;
    let mut elapsed: f64 = 0.0;

    'running: loop {
        start = timer.performance_counter();

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

        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.clear();

        let surface = font
            .render(&*format!("FPS: {:.2}", 1.0 / elapsed))
            .blended(Color::RGB(0, 255, 0))
            .map_err(|e| e.to_string())?;

        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(600, 0, 200, 60);
        canvas.copy(&texture, None, Some(target))?;
        canvas.present();

        end = timer.performance_counter();
        elapsed = (end - start) as f64 / timer.performance_frequency() as f64;

        timer.delay((16.666 - elapsed * 1000.0).floor() as u32);

        end = timer.performance_counter();
        elapsed = (end - start) as f64 / timer.performance_frequency() as f64;
    }

    Ok(())
}
