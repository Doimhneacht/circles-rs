use std::time::{Instant, Duration};

pub mod entities;

pub struct Game {
    time_played: f32,
    last_frame_elapsed: f32,
    last_frame_time: Instant,
    time_started: Instant,
}

impl Game {
    pub fn new() -> Game {
        let now = Instant::now();
        Game {
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
}

fn secs_duration(duration: Duration) -> f32 {
    duration.as_secs() as f32 + duration.subsec_nanos() as f32 / 1_000_000_000.0
}
