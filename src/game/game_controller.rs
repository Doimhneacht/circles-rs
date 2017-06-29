use super::game_data::GameData;

pub fn update(game_data: &mut GameData, time_delta: f32) {
    for food in &mut game_data.food {
        food.circle.update(time_delta);
    }
}
