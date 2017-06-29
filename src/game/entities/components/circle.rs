use rand;
use rand::Rng;
use cgmath::{Vector2, Vector4, Zero};

pub struct Circle {
    pub position: Vector2<f32>,
    pub radius: f32,
    pub time: f32,
    pub base_color: Vector4<f32>,
    pub new_color: Vector4<f32>,
}

impl Circle {
    // Construct new particles far away so they can't be seen initially
    pub fn new() -> Circle {
        Circle {
            position: Vector2::zero(),
            radius: 50.0,
            time: 0.0,
            base_color: Vector4::new(0.0, 0.0, 0.0, 1.0),
            new_color: Vector4::new(1.0, 1.0, 1.0, 1.0),
        }
    }

    pub fn new_randomized() -> Circle {
        let mut rng = rand::thread_rng();

        Circle {
            position: Vector2::new(rng.gen_range(-300.0, 300.0), rng.gen_range(-300.0, 300.0)),
            radius: rng.gen_range(10.0, 100.0),
            time: rand::random(),
            base_color: Vector4::new(rand::random(), rand::random(), rand::random(), 1.0),
            new_color: Vector4::new(rand::random(), rand::random(), rand::random(), 1.0),
        }
    }

    pub fn update(&mut self, time_delta: f32) {
        self.time += time_delta;
        if self.time > 1.0 {
            self.time %= 1.0;
            let temp = self.base_color;
            self.base_color = self.new_color;
            self.new_color = temp;
        }
    }
}
