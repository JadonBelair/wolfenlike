use anyhow::Result;
use image::math::Rect;
use image::{DynamicImage, GenericImageView};
use rayon::iter::IntoParallelIterator;
use rayon::prelude::ParallelIterator;
use winit::dpi::{LogicalSize, PhysicalSize};
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::EventLoop;
use winit::window::{Fullscreen, WindowBuilder};

use input::InputManager;
use renderer::Renderer;

const SCALE: i32 = 4;
const WIDTH: i32 = 180 * SCALE;
const HEIGHT: i32 = 135 * SCALE;

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
    walls: Vec<Vec<u32>>,
    floor: Vec<Vec<u32>>,
    ceiling: Vec<Vec<u32>>,
    textures: Vec<DynamicImage>,
}

#[derive(Default, Clone, Copy)]
struct Ray {
    ray_dir_x: f32,
    ray_dir_y: f32,
    side_dist_x: f32,
    side_dist_y: f32,
    delta_dist_x: f32,
    delta_dist_y: f32,
    map_x: i32,
    map_y: i32,
    side: i32,
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

    window
        .set_cursor_grab(winit::window::CursorGrabMode::Confined)
        .unwrap();
    window.set_cursor_visible(false);

    let renderer = Renderer::new(&window, WIDTH, HEIGHT)?;
    let input_manager = InputManager::new();
    let mut world = App::new(renderer, input_manager);

    world.push_texture(image::open("./images/Brick1a.png")?);
    world.push_texture(image::open("./images/Stone1.png")?);
    world.push_texture(image::open("./images/Stone4.png")?);

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

            if world.input_manager.is_just_pressed(VirtualKeyCode::Q)
                || world.input_manager.request_exit
            {
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
            textures: Vec::new(),
            renderer,
            input_manager,
            walls: vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
                vec![2, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 0, 1, 0, 0, 1, 0, 0, 1],
                vec![1, 0, 0, 1, 0, 0, 1, 0, 0, 1],
                vec![1, 0, 0, 1, 1, 1, 1, 0, 1, 1],
                vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 1, 0, 0, 0, 0, 0, 1],
                vec![1, 0, 0, 0, 0, 0, 0, 0, 0, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            ],
            floor: vec![
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
            ],
            ceiling: vec![
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
                vec![2, 3, 2, 3, 2, 3, 2, 3, 2, 3],
                vec![3, 2, 3, 2, 3, 2, 3, 2, 3, 2],
            ],
        }
    }

    fn push_texture(&mut self, texture: DynamicImage) -> usize {
        self.textures.push(texture);
        self.textures.len() - 1
    }

    /// Update the `World` internal state; bounce the box around the screen.
    fn update(&mut self) {
        if let Some(size) = self.input_manager.request_resize {
            self.renderer.resize(size);
        }

        let delta = self.input_manager.elapsed().unwrap().as_secs_f32();

        let turn_speed = {
            let (motion_x, _) = self.input_manager.mouse_motion();
            motion_x as f32 * delta * 2.0
        };
        let old_dir_x = self.dir_x;
        self.dir_x = self.dir_x * (turn_speed).cos() - self.dir_y * (turn_speed).sin();
        self.dir_y = old_dir_x * (turn_speed).sin() + self.dir_y * (turn_speed).cos();
        let old_plane_x = self.plane_x;
        self.plane_x = self.plane_x * (turn_speed).cos() - self.plane_y * (turn_speed).sin();
        self.plane_y = old_plane_x * (turn_speed).sin() + self.plane_y * (turn_speed).cos();

        let move_speed = 5.0 * delta;

        let mut move_x = 0.0;
        let mut move_y = 0.0;

        if self.input_manager.is_down(VirtualKeyCode::W) {
            move_x += self.dir_x;
            move_y += self.dir_y;
        }
        if self.input_manager.is_down(VirtualKeyCode::S) {
            move_x -= self.dir_x;
            move_y -= self.dir_y;
        }
        if self.input_manager.is_down(VirtualKeyCode::D) {
            move_y += self.dir_x;
            move_x -= self.dir_y;
        }
        if self.input_manager.is_down(VirtualKeyCode::A) {
            move_y -= self.dir_x;
            move_x += self.dir_y;
        }

        let dist = (move_x.powi(2) + move_y.powi(2)).sqrt();
        move_x = move_x / dist;
        move_y = move_y / dist;

        move_x *= move_speed;
        move_y *= move_speed;

        if self.walls[self.player_y as usize][(self.player_x + move_x) as usize] == 0 {
            self.player_x += move_x;
        }
        if self.walls[(self.player_y + move_y) as usize][self.player_x as usize] == 0 {
            self.player_y += move_y;
        }
    }

    fn render(&self) -> Result<()> {
        self.renderer.render()
    }

    /// Draw the `World` state to the frame buffer.
    fn draw(&mut self) {
        self.renderer.fill(&[0, 0, 0, 0xff]);

        // cast a ray for each pixel column
        let z_buffer = (0..WIDTH)
            .into_par_iter()
            .map(|x| {
                let camera_x = 2.0 * x as f32 / WIDTH as f32 - 1.0;
                let ray_dir_x = self.dir_x + self.plane_x * -camera_x;
                let ray_dir_y = self.dir_y + self.plane_y * -camera_x;
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

                // DDA algorithm
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

                    if map_y < 0
                        || map_y >= self.walls.len() as i32
                        || map_x < 0
                        || map_x >= self.walls[0].len() as i32
                        || self.walls[map_y as usize][map_x as usize] > 0
                    {
                        hit = 1;
                    }
                }

                Ray {
                    ray_dir_x,
                    ray_dir_y,
                    side_dist_x,
                    side_dist_y,
                    delta_dist_x,
                    delta_dist_y,
                    map_x,
                    map_y,
                    side,
                }
            })
            .collect::<Vec<Ray>>();

        for y in (HEIGHT / 2)..HEIGHT {
            let ray_dir_x0 = self.dir_x + self.plane_x;
            let ray_dir_y0 = self.dir_y + self.plane_y;
            let ray_dir_x1 = self.dir_x - self.plane_x;
            let ray_dir_y1 = self.dir_y - self.plane_y;

            // minimal division distance calculation
            let row_dist = HEIGHT as f32 / ((y << 1) as f32 - HEIGHT as f32);

            let floor_step_x = row_dist * (ray_dir_x1 - ray_dir_x0) / WIDTH as f32;
            let floor_step_y = row_dist * (ray_dir_y1 - ray_dir_y0) / WIDTH as f32;

            let mut floor_x = self.player_x + row_dist * ray_dir_x0;
            let mut floor_y = self.player_y + row_dist * ray_dir_y0;

            let shade = ((y - (HEIGHT >> 1)) << 1) as f32 / HEIGHT as f32;

            for x in 0..WIDTH {
                let cell_x = floor_x as i32;
                let cell_y = floor_y as i32;

                if let Some(Some(&id)) = self
                    .floor
                    .get(cell_y as usize)
                    .map(|row| row.get(cell_x as usize))
                {
                    if id > 0 {
                        let floor_texture = &self.textures[id as usize - 1];
                        let floor_tx = (floor_texture.width() as f32 * (floor_x - cell_x as f32))
                            as u32
                            & (floor_texture.width() - 1);
                        let floor_ty = (floor_texture.height() as f32 * (floor_y - cell_y as f32))
                            as u32
                            & (floor_texture.height() - 1);

                        let floor_color = floor_texture.get_pixel(floor_tx, floor_ty);
                        let floor_color = [
                            (floor_color[0] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (floor_color[1] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (floor_color[2] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (floor_color[3] as f32 * shade).clamp(0.0, 255.0) as u8,
                        ];
                        self.renderer.draw_pixel(&floor_color, x, y);
                    }
                }

                if let Some(Some(&id)) = self
                    .ceiling
                    .get(cell_y as usize)
                    .map(|row| row.get(cell_x as usize))
                {
                    if id > 0 {
                        let ceil_texture = &self.textures[id as usize - 1];
                        let ceil_tx = (ceil_texture.width() as f32 * (floor_x - cell_x as f32)) as u32
                            & (ceil_texture.width() - 1);
                        let ceil_ty = (ceil_texture.height() as f32 * (floor_y - cell_y as f32)) as u32
                            & (ceil_texture.height() - 1);

                        let ceil_color = ceil_texture.get_pixel(ceil_tx, ceil_ty);
                        let ceil_color = [
                            (ceil_color[0] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (ceil_color[1] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (ceil_color[2] as f32 * shade).clamp(0.0, 255.0) as u8,
                            (ceil_color[3] as f32 * shade).clamp(0.0, 255.0) as u8,
                        ];
                        self.renderer.draw_pixel(&ceil_color, x, HEIGHT - y);
                    }
                }

                floor_x += floor_step_x;
                floor_y += floor_step_y;
            }
        }

        for (x, ray) in z_buffer.iter().enumerate() {
            let ray_dir_x = ray.ray_dir_x;
            let ray_dir_y = ray.ray_dir_y;
            let side_dist_x = ray.side_dist_x;
            let side_dist_y = ray.side_dist_y;
            let delta_dist_x = ray.delta_dist_x;
            let delta_dist_y = ray.delta_dist_y;
            let map_x = ray.map_x;
            let map_y = ray.map_y;
            let side = ray.side;

            let texture_id = self
                .walls
                .get(map_y as usize)
                .map(|row| *row.get(map_x as usize).unwrap_or(&1))
                .unwrap_or(1) as usize
                - 1;
            let texture = &self.textures[texture_id];

            // correct fish-eye effect
            let perp_wall_dist = if side == 0 {
                side_dist_x - delta_dist_x
            } else {
                side_dist_y - delta_dist_y
            };

            // used to index into wall texture
            let mut wall_x = if side == 0 {
                self.player_y + perp_wall_dist * ray_dir_y
            } else {
                self.player_x + perp_wall_dist * ray_dir_x
            };
            wall_x -= wall_x.floor();

            let mut tex_x = (wall_x * texture.width() as f32) as u32;
            // unmirrors texture on certain walls
            if (side == 0 && ray_dir_x < 0.0) || (side == 1 && ray_dir_y > 0.0) {
                tex_x = texture.width() - tex_x - 1;
            }

            // ceiling the line height mostly removes an issue where there
            // will be a pixel of the floor/roof at the edges of the wall
            let line_height = (HEIGHT as f32 / perp_wall_dist).ceil() as i32;
            let top = HEIGHT / 2 - (line_height / 2);
            let color = if side == 0 { 0x99 } else { 0xff } as f32;
            let shade = 255.0 * (line_height as f32 / HEIGHT as f32);
            let color = (color * shade / 255.0).clamp(0.0, 255.0) as u8;

            let sub_image = Rect {
                x: tex_x,
                y: 0,
                width: 1,
                height: texture.height(),
            };

            self.renderer.draw_sub_texture(
                texture,
                &[color, color, color, 0xff],
                x as i32,
                top,
                PhysicalSize::new(1, line_height as u32),
                sub_image,
            );
        }
    }
}
