use image::{open, DynamicImage};
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent};
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use anyhow::Result;

use crate::renderer::Renderer;

const WIDTH: u32 = 320;
const HEIGHT: u32 = 180;

mod renderer;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct App {
    renderer: Renderer,
    offset: usize,
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f64, HEIGHT as f64);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let renderer = Renderer::new(&window, WIDTH, HEIGHT)?;
    let mut world = App::new(renderer);

    let bricks = open("./images/Brick1a.png")?;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        match event {
            Event::RedrawRequested(_) => {
                world.draw(&bricks);
                if let Err(_) = world.render() {
                    control_flow.set_exit();
                    return;
                }
            }
            Event::WindowEvent { event: WindowEvent::CloseRequested, .. } => {
                control_flow.set_exit();
            }
            Event::WindowEvent { event: WindowEvent::KeyboardInput { input: KeyboardInput { state: ElementState::Pressed, virtual_keycode: Some(keycode), ..}, ..}, ..} => {
                match keycode {
                    VirtualKeyCode::Escape | VirtualKeyCode::Q => control_flow.set_exit(),
                    _ => ()
                }
            }
            Event::WindowEvent { event: WindowEvent::Resized(size), .. } => {
                world.renderer.resize(size);
            }
            Event::MainEventsCleared => {
                world.update();
                window.request_redraw();
            }
            _ => ()
        }
    });
}

impl App {
    /// Create a new `World` instance that can draw a moving box.
    fn new(renderer: Renderer) -> Self {
        Self {
            offset: 1,
            renderer,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        self.offset += 1;
        self.offset %= WIDTH as usize;
    }

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self, bricks: &DynamicImage) {
        self.renderer.fill(&[0xff, 0xff, 0xff, 0xff]);
        self.renderer.draw_pixel(&[0, 0, 0xff, 0xff], 0, 0);
        self.renderer.draw_texture(bricks, 10, 10, PhysicalSize::new(100, 100));
        self.renderer.draw_vert_line(&[(self.offset % 256) as u8, 75, 75, 0xff], self.offset, 10, 160);
    }
}
