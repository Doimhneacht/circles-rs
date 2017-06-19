pub use self::inner::{Camera, MovableCamera};

mod inner {
    use cgmath::{Vector2, Basis2, Rotation, Rotation2, Zero, Deg};

    pub struct Camera {
        position: Vector2<f32>,
        state: CameraState,
    }

    pub struct CameraState {
        pub moving_up: bool,
        pub moving_right: bool,
        pub moving_down: bool,
        pub moving_left: bool,
    }

    pub trait MovableCamera: MovableCameraHelper {
        fn compute(&mut self, time: f32);

        fn mut_state(&mut self) -> &mut CameraState;
    }

    trait MovableCameraHelper {
        fn camera_speed(&self) -> Vector2<f32>;
    }

    impl MovableCamera for Camera {
        fn compute(&mut self, time: f32) {
            self.position += self.camera_speed() * time;
        }

        fn mut_state(&mut self) -> &mut CameraState { &mut self.state }
    }

    impl MovableCameraHelper for Camera {
        fn camera_speed(&self) -> Vector2<f32> {
            let angle = match (self.state.moving_up, self.state.moving_right, self.state.moving_down, self.state.moving_left) {
                (true, true, false, false) => 45.0,
                (true, false, false, true) => 135.0,
                (false, false, true, true) => 225.0,
                (false, true, true, false) => 315.0,
                (_, true, _, false) => 0.0,
                (true, _, false, _) => 90.0,
                (_, false, _, true) => 180.0,
                (false, _, true, _) => 270.0,
                _ => -1.0,
            };

            if angle == -1.0 {
                return Vector2::zero();
            }

            Basis2::from_angle(Deg(angle))
                .rotate_vector(Vector2::new(200.0, 0.0))
        }
    }

    impl Camera {
        pub fn new() -> Camera {
            Camera {
                position: Vector2::zero(),
                state: CameraState {
                    moving_up: false,
                    moving_right: false,
                    moving_down: false,
                    moving_left: false,
                }
            }
        }

        pub fn position(&self) -> Vector2<f32> { self.position }
    }
}
