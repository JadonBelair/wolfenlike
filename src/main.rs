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
    map: Vec<Vec<u32>>,
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
            player_x: 1.0,
            player_y: 1.0,
            player_angle: 0.0,
            renderer,
            input_manager,
            map: vec![
                vec![1,1,1,1,1,1,1,1,1,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,1,1,1,1,1,1,1,1,1],
            ],
        }
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if let Some(size) = self.input_manager.request_resize {
            self.renderer.resize(size);
        }


        let delta = self.input_manager.elapsed().unwrap().as_secs_f32();
        let speed = 1000.0 * delta;

        if self.input_manager.is_down(VirtualKeyCode::A) {
            self.player_angle += 1.5 * delta;
        }
        if self.input_manager.is_down(VirtualKeyCode::D) {
            self.player_angle -= 1.5 * delta;
        }
        if self.input_manager.is_down(VirtualKeyCode::W) {
            let delta_x = self.player_angle.sin() * speed * delta;
            let delta_y = self.player_angle.cos() * speed * delta;
            if self.map[self.player_y as usize][(self.player_x + delta_x) as usize] == 0 {
                self.player_x += delta_x;
            }
            if self.map[(self.player_y + delta_y) as usize][self.player_x as usize] == 0 {
                self.player_y += delta_y;
            }
        }
        if self.input_manager.is_down(VirtualKeyCode::S) {
            let delta_x = self.player_angle.sin() * speed * delta;
            let delta_y = self.player_angle.cos() * speed * delta;
            if self.map[self.player_y as usize][(self.player_x - delta_x) as usize] == 0 {
                self.player_x -= delta_x;
            }
            if self.map[(self.player_y - delta_y) as usize][self.player_x as usize] == 0 {
                self.player_y -= delta_y;
            }
        }
    }

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&mut self) {
        self.renderer.fill(&[0, 0, 0, 0xff]);

        for (y, row) in self.map.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell != 0 {
                    self.renderer.draw_rectangle(&[0xff, 0xff, 0xff, 0xff], x as i32 * 10, y as i32 * 10, 9, 9);
                }
            }
        }

        let speed = 15.0;
        let end_x = self.player_angle.sin() * speed;
        let end_y = self.player_angle.cos() * speed;

        self.renderer.draw_line(&[0x00, 0xff, 0x00, 0xff], (self.player_x * 10.0) as i32, (self.player_y  * 10.0) as i32, (self.player_x * 10.0) as i32 + end_x as i32, (self.player_y * 10.0) as i32 + end_y as i32);
        self.renderer.draw_rectangle(&[0x00, 0xff, 0x00, 0xff], (self.player_x * 10.0) as i32 - 2, (self.player_y * 10.0) as i32 - 2, 5, 5);
    }
}
