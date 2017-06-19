use std::time::{Instant, Duration};
use winit;

pub mod entities;
mod camera;

pub use self::camera::Camera;
use self::camera::MovableCamera;

pub struct Game {
    camera: Camera,
    player: entities::Circle,
    time_played: f32,
    last_frame_elapsed: f32,
    last_frame_time: Instant,
    time_started: Instant,
}

impl Game {
    pub fn new() -> Game {
        let now = Instant::now();
        Game {
            camera: Camera::new(),
            player: entities::Circle::new(),
            time_played: 0.0,
            last_frame_elapsed: 0.0,
            last_frame_time: now,
            time_started: now,
        }
    }

    pub fn update_time(&mut self) {
        let now = Instant::now();
        let delta = now - self.last_frame_time;
        self.last_frame_elapsed = secs_duration(now - self.last_frame_time);
        self.last_frame_time = now;
        self.time_played = secs_duration(now - self.time_started);
    }

    pub fn process_event(&mut self, event: &winit::WindowEvent) {
        match *event {
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::W), _) => {
                self.camera.mut_state().moving_up = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::D), _) => {
                self.camera.mut_state().moving_right = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::S), _) => {
                self.camera.mut_state().moving_down = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::A), _) => {
                self.camera.mut_state().moving_left = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::W), _) => {
                self.camera.mut_state().moving_up = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::D), _) => {
                self.camera.mut_state().moving_right = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::S), _) => {
                self.camera.mut_state().moving_down = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::A), _) => {
                self.camera.mut_state().moving_left = false;
            },
            _ => ()
        }
    }

    pub fn play(&mut self) {
        self.player.time += self.last_frame_elapsed;
        if self.player.time > 1.0 {
            self.player.time %= 1.0;
            self.player.swap_colors();
        }

        self.camera.compute(self.last_frame_elapsed);
    }

    pub fn camera(&self) -> &Camera { &self.camera }

    pub fn circle(&self) -> &entities::Circle { &self.player }
}

fn secs_duration(duration: Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}
