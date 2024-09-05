#![allow(dead_code)]

use std::time::{Duration, Instant};
use winit::{
    dpi::PhysicalSize,
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
};

#[derive(Default)]
pub struct InputManager {
    just_pressed: Vec<VirtualKeyCode>,
    held: Vec<VirtualKeyCode>,
    released: Vec<VirtualKeyCode>,
    pub request_exit: bool,
    pub request_resize: Option<PhysicalSize<u32>>,
    start_time: Option<Instant>,
    delta_time: Option<Duration>,
}

impl InputManager {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    /// processes the current event and returns true 
    /// when there are no more events to process
    pub fn process_event(&mut self, event: &Event<'_, ()>) -> bool {
        match event {
            Event::NewEvents(_) => {
                self.request_resize = None;
                self.request_exit = false;
                self.just_pressed.clear();
                self.released.clear();

                self.start_time = Some(Instant::now());
                self.delta_time = None;
                false
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                self.request_resize = Some(*size);
                false
            }
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                self.request_exit = true;
                false
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if !self.just_pressed.contains(&keycode) {
                    self.just_pressed.push(*keycode);
                    self.held.push(*keycode);
                }
                false
            }
            Event::WindowEvent {
                event:
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Released,
                                virtual_keycode: Some(keycode),
                                ..
                            },
                        ..
                    },
                ..
            } => {
                if !self.released.contains(keycode) {
                    self.released.push(*keycode);
                    self.held = self
                        .held
                        .iter()
                        .filter(|&key| key != keycode)
                        .map(|key| *key)
                        .collect::<Vec<VirtualKeyCode>>();
                }
                false
            }
            Event::MainEventsCleared => {
                self.delta_time = self.start_time.map(|time| time.elapsed());
                self.start_time = Some(Instant::now());

                true
            }
            _ => false,
        }
    }

    /// returns whether or not the given key was just pressed
    pub fn is_just_pressed(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed.contains(&key)
    }

    /// returns whether or not the given key is currently down
    pub fn is_down(&self, key: VirtualKeyCode) -> bool {
        self.just_pressed.contains(&key) || self.held.contains(&key)
    }

    /// returns whether or not the given key was just released
    pub fn is_just_released(&self, key: VirtualKeyCode) -> bool {
        self.released.contains(&key)
    }

    /// time between start of last 2 frames
    pub fn elapsed(&self) -> Option<Duration> {
        self.delta_time
    }
}
