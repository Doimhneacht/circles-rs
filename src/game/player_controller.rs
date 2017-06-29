use winit;
use cgmath::{Vector2, Zero, Basis2, Rotation, Rotation2, Deg};

use super::entities::{Player, PlayerState, ControllablePlayer};
use super::entities::components::Circle;

pub fn process_event(player: &mut ControllablePlayer, event: &winit::WindowEvent) {
    match *event {
        winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::W), _) => {
            player.mut_state().moving_up = true;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::D), _) => {
            player.mut_state().moving_right = true;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::S), _) => {
            player.mut_state().moving_down = true;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::A), _) => {
            player.mut_state().moving_left = true;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::W), _) => {
            player.mut_state().moving_up = false;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::D), _) => {
            player.mut_state().moving_right = false;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::S), _) => {
            player.mut_state().moving_down = false;
        },
        winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::A), _) => {
            player.mut_state().moving_left = false;
        },
        _ => ()
    }
}

pub fn update(mut player: &mut Player, time_delta: f32) {
    update_position(&mut player, time_delta);
    player.circle.update(time_delta);
}

fn update_position(player: &mut Player, time_delta: f32) {
    player.circle.position += player_speed(player.mut_state()) * time_delta;
}

fn player_speed(player_state: &PlayerState) -> Vector2<f32> {
    let angle = match (player_state.moving_up, player_state.moving_right, player_state.moving_down, player_state.moving_left) {
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
