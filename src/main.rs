extern crate rand;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate winit;
extern crate glutin;
extern crate gfx_window_glutin;

use gfx::Device;

mod game;
mod pipeline;
mod visualizer;

use game::Game;

use pipeline::{ColorFormat, DepthFormat};

pub fn main() {
    std::env::set_var("RUST_BACKTRACE", "1");

    let builder = glutin::WindowBuilder::new()
        .with_title("circles-rs".to_string())
        .with_dimensions(1280, 720)
        .with_vsync();

    let events_loop = glutin::EventsLoop::new();

    let (window, mut device, mut factory, main_color, main_depth) =
        gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, &events_loop);
    let mut encoder: gfx::Encoder<_, _> = factory.create_command_buffer().into();
    let mut visualizer = visualizer::Visualizer::new(factory, main_color, main_depth);

    let mut game = Game::new();

    let mut running = true;
    while running {
        game.update_time();

        events_loop.poll_events(|winit::Event::WindowEvent{window_id: _, event}| {
            match event {
                winit::WindowEvent::Closed => { running = false },
                _ => { game.process_event(&event) },
            }
        });

        game.play();

        visualizer.render(&mut encoder, game.camera(), game.circle());
        encoder.flush(&mut device);
        window.swap_buffers().unwrap();
        device.cleanup();
    }
}
