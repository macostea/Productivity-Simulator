extern crate sdl2;
mod framerate;

use std::path::PathBuf;

use framerate::Framerate;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use assets_manager::{source::{FileSystem, DirEntry}};

fn get_framerate_font_path<'a>() -> Result<PathBuf, String> {
    let fs = FileSystem::new("assets").map_err(|e| e.to_string())?;
    let font_file = DirEntry::File("fonts.OpenSans-Regular", "ttf");
    let font_fs_path = fs.path_of(font_file);
    Ok(font_fs_path)
}

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

    let font_path = get_framerate_font_path()?;
    let mut framerate = Framerate::new(&sdl_context, &ttf_context, canvas.texture_creator(), font_path.as_path())?;

    let mut event_pump = sdl_context.event_pump().map_err(|e| e.to_string())?;

    'running: loop {
        framerate.frame_start();

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

        framerate.draw(&mut canvas)?;

        canvas.present();
        framerate.frame_end();
    }

    Ok(())
}
