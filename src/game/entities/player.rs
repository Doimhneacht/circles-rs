use super::components;

pub struct Player {
    pub circle: components::Circle,
    state: PlayerState,
}

pub struct PlayerState {
    pub moving_up: bool,
    pub moving_right: bool,
    pub moving_down: bool,
    pub moving_left: bool,
}

pub trait ControllablePlayer {
    fn mut_state(&mut self) -> &mut PlayerState;
}

impl ControllablePlayer for Player {
    fn mut_state(&mut self) -> &mut PlayerState { &mut self.state }
}

impl Player {
    pub fn new() -> Player {
        Player {
            circle: components::Circle::new(),
            state: PlayerState {
                moving_up: false,
                moving_right: false,
                moving_down: false,
                moving_left: false,
            }
        }
    }
}
