use super::components;

pub struct Food {
    pub circle: components::Circle,
    state: FoodState,
}

pub struct FoodState {
}

pub trait ControllableFood {
    fn mut_state(&mut self) -> &mut FoodState;
}

impl ControllableFood for Food {
    fn mut_state(&mut self) -> &mut FoodState { &mut self.state }
}

impl Food {
    pub fn new() -> Food {
        Food {
            circle: components::Circle::new_randomized(),
            state: FoodState {
            }
        }
    }
}
