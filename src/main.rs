use anyhow::Result;
use image::math::Rect;
use image::open;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, WindowBuilder};

use input::InputManager;
use renderer::Renderer;

const WIDTH: i32 = 320;
const HEIGHT: i32 = 180;

mod input;
mod renderer;

/// Representation of the application state. In this example, a box will bounce around the screen.
struct App {
    renderer: Renderer,
    player_x: f32,
    player_y: f32,
    player_angle: i32,
}

fn main() -> Result<()> {
    let event_loop = EventLoop::new();
    let window = {
        let size = LogicalSize::new(1280.0, 720.0);
        WindowBuilder::new()
            .with_title("Hello Pixels")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    let renderer = Renderer::new(&window, WIDTH, HEIGHT)?;
    let mut world = App::new(renderer);
    let mut input_helper = InputManager::new();

    let bricks = open("./images/Brick4a.png")?;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        if let Event::RedrawRequested(_) = event {
            world.draw();
            world.renderer.draw_texture(&bricks, 10, 10, PhysicalSize::new(64, 64));
            world.renderer.draw_texture(&bricks, 42, 10, PhysicalSize::new(64, 64));
            world.renderer.draw_texture(&bricks, 74, 10, PhysicalSize::new(64, 64));
            world.renderer.draw_texture(&bricks, 106, 10, PhysicalSize::new(64, 64));
            world.renderer.draw_sub_texture(&bricks, 10, 94, PhysicalSize::new(10, 64), Rect { x: 0, y: 0, width: 1, height: 64 });
            if let Err(_) = world.render() {
                control_flow.set_exit();
                return;
            }
        }

        // main loop logic
        if input_helper.process_event(&event) {
            println!("{:?}", input_helper.elapsed().unwrap());
            if let Some(size) = input_helper.request_resize {
                world.renderer.resize(size);
            }

            if input_helper.is_just_pressed(VirtualKeyCode::F) {
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }
            }

            if input_helper.is_just_pressed(VirtualKeyCode::Q) || input_helper.request_exit {
                control_flow.set_exit();
            }

            let delta = input_helper.elapsed().unwrap();
            let speed = 5000.0 * delta.as_secs_f32();

            if input_helper.is_down(VirtualKeyCode::A) {
                world.player_x -= speed;
            }
            if input_helper.is_down(VirtualKeyCode::D) {
                world.player_x += speed;
            }
            if input_helper.is_down(VirtualKeyCode::W) {
                world.player_y -= speed;
            }
            if input_helper.is_down(VirtualKeyCode::S) {
                world.player_y += speed;
            }

            world.update();
            window.request_redraw();
        }
    });
}

impl App {
    /// Create a new `World` instance that can draw a moving box.
    fn new(renderer: Renderer) -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_angle: 0,
            renderer,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {}

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    ///
    /// Assumes the default texture format: `wgpu::TextureFormat::Rgba8UnormSrgb`
    fn draw(&mut self) {
        self.renderer.fill(&[0xff, 0xff, 0xff, 0xff]);
        self.renderer.draw_pixel(&[0, 0, 0xff, 0xff], 0, 0);
        self.renderer
            .draw_rectangle(&[0x00, 0xff, 0xff, 0xff], 10, 116, 64, 64);
        self.renderer.draw_rectangle(
            &[0xff, 0x00, 0xff, 0xff],
            self.player_x as i32,
            self.player_y as i32,
            20,
            20,
        );
    }
}
