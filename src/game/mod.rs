use std::time::{Instant, Duration};
use std::vec::Vec;
use winit;

pub mod entities;
pub mod game_data;
mod game_controller;
mod player_controller;

pub struct Game {
    game_data: game_data::GameData,
    time_played: f32,
    last_frame_elapsed: f32,
    last_frame_time: Instant,
    time_started: Instant,
}

impl Game {
    pub fn new() -> Game {
        let now = Instant::now();
        Game {
            game_data: game_data::GameData::new(),
            time_played: 0.0,
            last_frame_elapsed: 0.0,
            last_frame_time: now,
            time_started: now,
        }
    }

    pub fn update_time(&mut self) {
        let now = Instant::now();
        self.last_frame_elapsed = secs_duration(now - self.last_frame_time);
        self.last_frame_time = now;
        self.time_played = secs_duration(now - self.time_started);
    }

    pub fn process_event(&mut self, event: &winit::WindowEvent) {
        player_controller::process_event(&mut self.game_data.player, &event);
    }

    pub fn play(&mut self) {
        player_controller::update(&mut self.game_data.player, self.last_frame_elapsed);
        game_controller::update(&mut self.game_data, self.last_frame_elapsed);
    }

    pub fn player(&self) -> &entities::Player { &self.game_data.player }

    pub fn food(&self) -> &Vec<entities::Food> { &self.game_data.food }
}

fn secs_duration(duration: Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}
