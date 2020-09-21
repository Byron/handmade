pub mod canvas;

use image::*;

#[derive(Debug)]
pub struct PageProps<'a> {
    pub canvas: &'a canvas::CanvasProps<'a>,
    pub margins: f32,
    pub line_height: f32,
}

impl PageProps<'_> {
    pub fn line_max_width(&self) -> f32 {
        self.canvas.width as f32 - (self.margins * 2.0)
    }

    pub fn white_page(&self) -> RgbImage {
        ImageBuffer::from_pixel(self.canvas.width, self.canvas.height, image::Rgb([255, 255, 255]))
    }
}

pub static DEFAULT : PageProps = PageProps { line_height: 35.0, margins: 10.0, canvas: &canvas::A4 };
