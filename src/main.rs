#![allow(dead_code)]

use anyhow::Result;
use winit::dpi::LogicalSize;
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
    input_manager: InputManager,
    player_x: f32,
    player_y: f32,
    player_angle: f32,
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
    let input_manager = InputManager::new();
    let mut world = App::new(renderer, input_manager);

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        if let Event::RedrawRequested(_) = event {
            world.draw();
            if let Err(_) = world.render() {
                control_flow.set_exit();
                return;
            }
        }

        // main loop logic
        if world.input_manager.process_event(&event) {
            if world.input_manager.is_just_pressed(VirtualKeyCode::F) {
                if window.fullscreen().is_some() {
                    window.set_fullscreen(None);
                } else {
                    window.set_fullscreen(Some(Fullscreen::Borderless(None)));
                }
            }

            if world.input_manager.is_just_pressed(VirtualKeyCode::Q) || world.input_manager.request_exit {
                control_flow.set_exit();
            }

            world.update();
            window.request_redraw();
        }
    });
}

impl App {
    /// Create a new `World` instance that can draw a moving box.
    fn new(renderer: Renderer, input_manager: InputManager) -> Self {
        Self {
            player_x: 0.0,
            player_y: 0.0,
            player_angle: 0.0,
            renderer,
            input_manager,
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if let Some(size) = self.input_manager.request_resize {
            self.renderer.resize(size);
        }


        let delta = self.input_manager.elapsed().unwrap();
        let speed = 5000.0 * delta.as_secs_f32();

        if self.input_manager.is_down(VirtualKeyCode::A) {
            self.player_angle += 10000.0 * delta.as_secs_f32();
            if self.player_angle < 0.0 {
                self.player_angle += 360.0;
            }
            if self.player_angle > 360.0 {
                self.player_angle -= 360.0;
            }
        }
        if self.input_manager.is_down(VirtualKeyCode::D) {
            self.player_angle -= 10000.0 * delta.as_secs_f32();
            if self.player_angle < 0.0 {
                self.player_angle += 360.0;
            }
            if self.player_angle > 360.0 {
                self.player_angle -= 360.0;
            }
        }
        if self.input_manager.is_down(VirtualKeyCode::W) {
            let mut delta_y = (speed * self.player_angle.to_radians().sin()).abs();
            let mut delta_x = (speed.powi(2) - delta_y.powi(2)).sqrt();

            if self.player_angle >= 0.0 && self.player_angle <= 180.0 {
                delta_y *= -1.0;
            }

            if self.player_angle > 90.0 && self.player_angle <= 270.0 {
                delta_x *= -1.0
            }

            self.player_y += delta_y;
            self.player_x += delta_x;
        }
        if self.input_manager.is_down(VirtualKeyCode::S) {
            let mut delta_y = (speed * self.player_angle.to_radians().sin()).abs();
            let mut delta_x = (speed.powi(2) - delta_y.powi(2)).sqrt();

            if self.player_angle >= 0.0 && self.player_angle <= 180.0 {
                delta_y *= -1.0;
            }

            if self.player_angle > 90.0 && self.player_angle <= 270.0 {
                delta_x *= -1.0
            }

            self.player_y -= delta_y;
            self.player_x -= delta_x;
        }
    }

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&mut self) {
        self.renderer.fill(&[0, 0, 0, 0xff]);

        let speed = 15.0;
        let mut end_y= (speed * self.player_angle.to_radians().sin()).abs();
        let mut end_x = (speed.powi(2) - end_y.powi(2)).sqrt();

        if self.player_angle >= 0.0 && self.player_angle <= 180.0 {
            end_y *= -1.0;
        }

        if self.player_angle > 90.0 && self.player_angle <= 270.0 {
            end_x *= -1.0
        }

        self.renderer.draw_line(&[0x00, 0xff, 0x00, 0xff], self.player_x as i32 + 3, self.player_y as i32 + 3, self.player_x as i32 + 3 + end_x as i32, self.player_y as i32 + 3 + end_y as i32);

        self.renderer.draw_rectangle(&[0x00, 0xff, 0x00, 0xff], self.player_x as i32, self.player_y as i32, 6, 6);
    }
}
