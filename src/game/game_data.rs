use std::vec::Vec;

use super::entities::{Player, Food};

const FOOD_DENSITY: u32 = 6;

pub struct GameData {
    pub player: Player,
    pub food: Vec<Food>,
    pub circle_density: u32,
    pub time_delta: u32,
}

impl GameData {
    pub fn new() -> GameData {
        let mut food: Vec<Food> = Vec::new();
        for _ in 0..FOOD_DENSITY {
            food.push(Food::new())
        }

        GameData {
            player: Player::new(),
            food: food,
            circle_density: FOOD_DENSITY,
            time_delta: 0,
        }
    }
}
