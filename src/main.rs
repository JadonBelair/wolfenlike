#![allow(dead_code)]

use anyhow::Result;
use image::math::Rect;
use image::DynamicImage;
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
    input_manager: InputManager,
    player_x: f32,
    player_y: f32,
    dir_x: f32,
    dir_y: f32,
    plane_x: f32,
    plane_y: f32,
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

    let bricks = image::open("./images/Brick1a.png")?;

    event_loop.run(move |event, _, control_flow| {
        control_flow.set_poll();

        if let Event::RedrawRequested(_) = event {
            world.draw(&bricks);
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
            player_x: 1.5,
            player_y: 1.5,
            dir_x: -1.0,
            dir_y: 0.0,
            plane_x: 0.0,
            plane_y: 0.66,
            renderer,
            input_manager,
            map: vec![
                vec![1,1,1,1,1,1,1,1,1,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,0,0,0,0,0,0,0,0,1],
                vec![1,1,0,1,0,0,1,0,0,1],
                vec![1,0,0,1,0,0,1,0,0,1],
                vec![1,0,0,1,1,1,1,0,1,1],
                vec![1,0,0,1,0,0,0,0,0,1],
                vec![1,0,0,1,0,0,0,0,0,1],
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
        let speed = 5.0 * delta;

        if self.input_manager.is_down(VirtualKeyCode::A) {
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * (1.5 * delta).cos() - self.dir_y * (1.5 * delta).sin();
            self.dir_y = old_dir_x * (1.5 * delta).sin() + self.dir_y * (1.5 * delta).cos();
            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * (1.5 * delta).cos() - self.plane_y * (1.5 * delta).sin();
            self.plane_y = old_plane_x * (1.5 * delta).sin() + self.plane_y * (1.5 * delta).cos();
        }
        if self.input_manager.is_down(VirtualKeyCode::D) {
            let old_dir_x = self.dir_x;
            self.dir_x = self.dir_x * (-1.5 * delta).cos() - self.dir_y * (-1.5 * delta).sin();
            self.dir_y = old_dir_x * (-1.5 * delta).sin() + self.dir_y * (-1.5 * delta).cos();
            let old_plane_x = self.plane_x;
            self.plane_x = self.plane_x * (-1.5 * delta).cos() - self.plane_y * (-1.5 * delta).sin();
            self.plane_y = old_plane_x * (-1.5 * delta).sin() + self.plane_y * (-1.5 * delta).cos();
        }
        if self.input_manager.is_down(VirtualKeyCode::W) {
            if self.map[self.player_y as usize][(self.player_x + self.dir_x * speed) as usize] == 0 {
                self.player_x += self.dir_x * speed;
            }
            if self.map[(self.player_y + self.dir_y * speed) as usize][self.player_x as usize] == 0 {
                self.player_y += self.dir_y * speed;
            }
        }
        if self.input_manager.is_down(VirtualKeyCode::S) {
            if self.map[self.player_y as usize][(self.player_x - self.dir_x * speed) as usize] == 0 {
                self.player_x -= self.dir_x * speed;
            }
            if self.map[(self.player_y - self.dir_y * speed) as usize][self.player_x as usize] == 0 {
                self.player_y -= self.dir_y * speed;
            }
        }
    }

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&mut self, bricks: &DynamicImage) {
        self.renderer.fill(&[0, 0, 0, 0xff]);

        for x in 0..WIDTH {
            let camera_x = 2.0 * x as f32 / WIDTH as f32 - 1.0;
            let ray_dir_x = self.dir_x + self.plane_x * camera_x;
            let ray_dir_y = self.dir_y + self.plane_y * camera_x;
            let mut map_x = self.player_x as i32;
            let mut map_y = self.player_y as i32;

            let delta_dist_x = (1.0 / ray_dir_x).abs();
            let delta_dist_y = (1.0 / ray_dir_y).abs();

            let mut hit = 0;
            let mut side = 0;

            let (step_x, mut side_dist_x) = if ray_dir_x < 0.0 {
                (-1, (self.player_x - map_x as f32) * delta_dist_x)
            } else {
                (1, (map_x as f32 + 1.0 - self.player_x) * delta_dist_x)
            };
            let (step_y, mut side_dist_y) = if ray_dir_y < 0.0 {
                (-1, (self.player_y - map_y as f32) * delta_dist_y)
            } else {
                (1, (map_y as f32 + 1.0 - self.player_y) * delta_dist_y)
            };

            while hit == 0 {
                if side_dist_x < side_dist_y {
                    side_dist_x += delta_dist_x;
                    map_x += step_x;
                    side = 0;
                } else {
                    side_dist_y += delta_dist_y;
                    map_y += step_y;
                    side = 1;
                }

                if self.map[map_y as usize][map_x as usize] > 0 {
                    hit = 1;
                }
            }

            let perp_wall_dist = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            let mut wall_x = if side == 0 {
                self.player_y + perp_wall_dist * ray_dir_y
            } else {
                self.player_x + perp_wall_dist * ray_dir_x
            };

            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * bricks.width() as f32) as u32;
            if (side == 0 && ray_dir_x > 0.0) 
                || (side == 1 && ray_dir_y < 0.0) {
                tex_x = bricks.width() - tex_x - 1;
            }

            let line_height = (HEIGHT as f32 / perp_wall_dist) as i32;
            let top = HEIGHT / 2 - (line_height / 2);
            let color = if side == 0 {
                0x99
            } else {
                0xff
            };

            self.renderer.draw_vert_line(&[color, color, color, 0xff], x, top, line_height);
            self.renderer.draw_sub_texture(bricks, x, top, PhysicalSize::new(1, line_height as u32), Rect {
                x: tex_x,
                y: 0,
                width: 1,
                height: bricks.height(),
            });

            self.renderer.draw_vert_line(&[0x00, 0xff, 0xff, 0xff], x, 0, top);
            self.renderer.draw_vert_line(&[0xff, 0x00, 0x00, 0xff], x, top+line_height, HEIGHT - (top+line_height));

        }
    }
}
