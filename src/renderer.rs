#![allow(dead_code)]

use anyhow::Result;
use image::{DynamicImage, GenericImageView};
use pixels::{Pixels, SurfaceTexture};
use winit::{dpi::PhysicalSize, window::Window};

pub struct Renderer {
    width: i32,
    height: i32,
    pub frame_buffer: Pixels,
}

impl Renderer {
    pub fn new(window: &Window, width: i32, height: i32) -> Result<Self> {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Ok(Self {
            width,
            height,
            frame_buffer: Pixels::new(width as u32, height as u32, surface_texture)?,
        })
    }

    /// renders the pixel buffer to the screen texture
    pub fn render(&self) -> Result<()> {
        Ok(self.frame_buffer.render()?)
    }

    /// resizes the pixel buffer to the nearest integer scale
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        self.frame_buffer
            .resize_surface(size.width, size.height)
            .expect("failed to resize surface");
    }

    /// fills the frame with the given color
    pub fn fill(&mut self, color: &[u8; 4]) {
        for pixel in self.frame_buffer.frame_mut().chunks_exact_mut(4) {
            pixel.copy_from_slice(color);
        }
    }

    /// draws a vertical line starting at the given x and top y with the given height
    pub fn draw_vert_line(&mut self, color: &[u8; 4], x: i32, top_y: i32, height: i32) -> i32 {
        if x < 0 || x >= self.width as i32 {
            return -1;
        }

        let actual_height = if top_y + height > self.height as i32 {
            self.height as i32 - top_y
        } else if top_y < 0 {
            (top_y + height).max(0)
        } else {
            height
        };

        for pixel in self
            .frame_buffer
            .frame_mut()
            .chunks_exact_mut(4)
            .skip(
                (x.clamp(0, self.width - 1)
                    + self.width * top_y.clamp(0, self.height)) as usize,
            )
            .step_by(self.width as usize)
            .take(actual_height as usize)
        {
            pixel.copy_from_slice(color);
        }

        actual_height
    }

    /// draws a colored rectangle at the specified x,y coords with the given size
    pub fn draw_rectangle(&mut self, color: &[u8; 4], x: i32, y: i32, width: i32, height: i32) {
        for i in 0..width {
            let _ = self.draw_vert_line(color, i + x, y, height);
        }
    }

    /// draws a colored pixel at the given x,y coordinates
    pub fn draw_pixel(&mut self, color: &[u8; 4], x: i32, y: i32) {
        if x < 0 || x > self.width || y < 0 || y >= self.height {
            return
        }

        let offset = ((y * self.width as i32 + x) * 4) as usize;
        let frame = self.frame_buffer.frame_mut();
        frame[offset + 0] = color[0];
        frame[offset + 1] = color[1];
        frame[offset + 2] = color[2];
        frame[offset + 3] = color[3];
    }

    /// draws the given image at the specified x,y coords with the given size
    pub fn draw_texture(
        &mut self,
        texture: &DynamicImage,
        x: i32,
        y: i32,
        size: PhysicalSize<u32>,
    ) {
        let x_scale = texture.width() as f32 / size.width as f32;
        let y_scale = texture.height() as f32 / size.height as f32;
        for c_y in 0..size.height {
            let offset_y = c_y as i32 + y;

            for c_x in 0..size.width {
                let offset_x = c_x as i32 + x;

                if (offset_x < 0 || offset_x >= self.width as i32)
                    || (offset_y < 0 || offset_y >= self.height as i32)
                {
                    continue;
                }
                let pix =
                    texture.get_pixel((c_x as f32 * x_scale) as u32, (c_y as f32 * y_scale) as u32);
                self.draw_pixel(&pix.0, offset_x, offset_y);
            }
        }
    }
}
