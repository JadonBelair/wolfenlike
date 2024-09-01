use image::{DynamicImage, GenericImageView, Pixel};
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};
use anyhow::Result;

pub struct Renderer {
    width: usize,
    height: usize,
    pub frame_buffer: Pixels,
}

impl Renderer {
    pub fn new(window: &Window, width: u32, height: u32) -> Result<Self> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Ok(Self {
            width: width as usize,
            height: height as usize,
            frame_buffer: Pixels::new(width, height, surface_texture)?,
        })
    }

    pub fn render(&self) -> Result<()> {
        Ok(self.frame_buffer.render()?)
    }

    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.frame_buffer.resize_surface(size.width, size.height).expect("failed to resize surface");
    }

    /// fills the frame with the given color
    pub fn fill(&mut self, color: &[u8; 4]) {
        for pixel in self.frame_buffer.frame_mut().chunks_exact_mut(4) {
            pixel.copy_from_slice(color);
        }
    }

    /// draws a vertical line starting at the given x and top y with the given height
    pub fn draw_vert_line(&mut self, color: &[u8; 4], x: usize, top_y: usize, height: usize) {
        for pixel in self.frame_buffer.frame_mut().chunks_exact_mut(4).skip(x+ (self.width * top_y)).step_by(self.width).take(height) {
            pixel.copy_from_slice(color);
        }
    }

    /// draws a colored pixel at the given x,y coordinates
    pub fn draw_pixel(&mut self, color: &[u8; 4], x: usize, y: usize) {
        let offset = (y * self.width + x) * 4;
        let frame = self.frame_buffer.frame_mut();
        frame[offset + 0] = color[0];
        frame[offset + 1] = color[1];
        frame[offset + 2] = color[2];
        frame[offset + 3] = color[3];
    }

    pub fn draw_texture(&mut self, texture: &DynamicImage, x: usize, y: usize, size: PhysicalSize<u32>) {
        let resized = texture.resize_exact(size.width, size.height, image::imageops::FilterType::Nearest);
        for (c_x, c_y, color) in resized.pixels() {
            self.draw_pixel(&color.0, c_x as usize + x, c_y as usize + y);
        }
    }
}
