#![allow(dead_code)]

use anyhow::Result;
use image::{math::Rect, DynamicImage, GenericImageView};
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
        self.frame_buffer.frame_mut().copy_from_slice(&color.repeat((self.width * self.height) as usize));
    }

    /// draws a vertical line starting at the given x and top y with the given height
    pub fn draw_vert_line(&mut self, color: &[u8; 4], x: i32, top_y: i32, height: i32) -> i32 {
        if x < 0 || x >= self.width || top_y >= self.height || top_y + height < 0 {
            return -1;
        }

        let actual_height = if top_y + height > self.height {
            self.height - top_y
        } else if top_y < 0 {
            (top_y + height).max(0)
        } else {
            height
        };

        for pixel in self
            .frame_buffer
            .frame_mut()
            .chunks_exact_mut(4)
            .skip((x.clamp(0, self.width - 1) + self.width * top_y.clamp(0, self.height)) as usize)
            .step_by(self.width as usize)
            .take(actual_height as usize)
        {
            pixel.copy_from_slice(color);
        }

        actual_height
    }

    /// draws a horizontal line starting at the given left x and y with the given width
    pub fn draw_hori_line(&mut self, color: &[u8; 4], left_x: i32, y: i32, width: i32) -> i32 {
        if y < 0 || y >= self.height || left_x >= self.width || left_x + width < 0 {
            return -1;
        }

        let actual_width = if left_x + width > self.width {
            self.width - left_x
        } else if left_x < 0 {
            (left_x + width).max(0)
        } else {
            width
        };

        for pixel in self
            .frame_buffer
            .frame_mut()
            .chunks_exact_mut(4)
            .skip((left_x.clamp(0, self.width - 1) + self.width * y.clamp(0, self.height)) as usize)
            .take(actual_width as usize)
        {
            pixel.copy_from_slice(color);
        }

        actual_width
    }

    /// draw a colored line between 2 given x,y points
    pub fn draw_line(&mut self, color: &[u8; 4], x1: i32, y1: i32, x2: i32, y2: i32) {
        if (x1 < 0 && x2 < 0)
            || (x1 >= self.width && x2 >= self.width)
            || (y1 < 0 && y2 < 0)
            || (y1 >= self.height && y2 >= self.height)
        {
            return;
        }

        if (y2 - y1).abs() < (x2 - x1).abs() {
            if x1 > x2 {
                self.draw_line_low(color, x2, y2, x1, y1);
            } else {
                self.draw_line_low(color, x1, y1, x2, y2);
            }
        } else {
            if y1 > y2 {
                self.draw_line_high(color, x2, y2, x1, y1);
            } else {
                self.draw_line_high(color, x1, y1, x2, y2);
            }
        }
    }

    fn draw_line_high(&mut self, color: &[u8; 4], x1: i32, y1: i32, x2: i32, y2: i32) {
        let frame = self.frame_buffer.frame_mut();

        let mut dx = x2 - x1;
        let dy = y2 - y1;
        let mut xi = 1;

        if dx < 0 {
            xi = -1;
            dx = -dx;
        }

        let mut d = (2 * dx) - dy;
        let mut x = x1;

        for y in y1..=y2 {
            if x >= 0 && x < self.width && y >= 0 && y < self.height {
                let offset = ((y * self.width + x) * 4) as usize;
                frame[offset + 0] = color[0];
                frame[offset + 1] = color[1];
                frame[offset + 2] = color[2];
                frame[offset + 3] = color[3];
            }

            if d > 0 {
                x += xi;
                d += 2 * (dx - dy);
            } else {
                d += 2 * dx;
            }
        }
    }

    fn draw_line_low(&mut self, color: &[u8; 4], x1: i32, y1: i32, x2: i32, y2: i32) {
        let frame = self.frame_buffer.frame_mut();

        let dx = x2 - x1;
        let mut dy = y2 - y1;
        let mut yi = 1;

        if dy < 0 {
            yi = -1;
            dy = -dy;
        }

        let mut d = (2 * dy) - dx;
        let mut y = y1;

        for x in x1..=x2 {
            if x >= 0 && x < self.width && y >= 0 && y < self.height {
                let offset = ((y * self.width + x) * 4) as usize;
                frame[offset + 0] = color[0];
                frame[offset + 1] = color[1];
                frame[offset + 2] = color[2];
                frame[offset + 3] = color[3];
            }

            if d > 0 {
                y += yi;
                d += 2 * (dy - dx);
            } else {
                d += 2 * dy;
            }
        }
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
            return;
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

    pub fn draw_sub_texture(
        &mut self,
        texture: &DynamicImage,
        x: i32,
        y: i32,
        size: PhysicalSize<u32>,
        sub_image: Rect,
    ) {
        let subimage = texture.view(sub_image.x, sub_image.y, sub_image.width, sub_image.height);

        let x_scale = subimage.width() as f32 / size.width as f32;
        let y_scale = subimage.height() as f32 / size.height as f32;
        for c_y in 0..size.height {
            let offset_y = c_y as i32 + y;

            for c_x in 0..size.width {
                let offset_x = c_x as i32 + x;

                if (offset_x < 0 || offset_x >= self.width as i32)
                    || (offset_y < 0 || offset_y >= self.height as i32)
                {
                    continue;
                }
                let pix = subimage
                    .get_pixel((c_x as f32 * x_scale) as u32, (c_y as f32 * y_scale) as u32);
                self.draw_pixel(&pix.0, offset_x, offset_y);
            }
        }
    }
}
