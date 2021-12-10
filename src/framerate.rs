use std::path::Path;

use sdl2::TimerSubsystem;
use sdl2::ttf::{Sdl2TtfContext, Font};
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::Sdl;

pub struct Framerate<'a> {
    timer: TimerSubsystem,
    start: u64,
    end: u64,
    elapsed: f64,
    font: Font<'a, 'a>,
    texture_creator: TextureCreator<WindowContext>
}

impl<'a> Framerate<'a> {
    pub fn new(sdl_context: &Sdl, ttf_context: &'a Sdl2TtfContext, texture_creator: TextureCreator<WindowContext>, font_path: &Path) -> Result<Framerate<'a>, String> {
        let font = ttf_context.load_font(font_path, 128)?;
    
        let timer = sdl_context.timer()?;
    
        Ok(Framerate { timer, start: 0, end: 0, elapsed: 0.0, font, texture_creator })
    }

    pub fn frame_start(&mut self) {
        self.start = self.timer.performance_counter();
    }

    pub fn draw(&self, canvas: &'a mut Canvas<Window>) -> Result<(), String> {
        let surface = self.font
            .render(&*format!("FPS: {:.2}", 1.0 / self.elapsed))
            .blended(Color::RGB(0, 255, 0))
            .map_err(|e| e.to_string())?;

        let texture = self.texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;

        let target = Rect::new(600, 0, 200, 60);
        canvas.copy(&texture, None, Some(target))?;

        Ok(())
    }

    pub fn frame_end(&mut self) {
        self.end = self.timer.performance_counter();
        self.elapsed = (self.end - self.start) as f64 / self.timer.performance_frequency() as f64;

        self.timer.delay((16.666 - self.elapsed * 1000.0).floor() as u32);

        self.end = self.timer.performance_counter();
        self.elapsed = (self.end - self.start) as f64 / self.timer.performance_frequency() as f64;
    }
}
