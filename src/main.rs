extern crate rand;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;
extern crate winit;

use std::time::Instant;
use std::ops::Neg;

use gfx_app::ColorFormat;
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;
use cgmath::{Matrix4, Vector3, Vector2, Basis2, ElementWise, Zero, Rotation2, Rotation, Rad, Deg};

// Declare the vertex format suitable for drawing,
// as well as the constants used by the shaders
// and the pipeline state object format.
gfx_defines! {
    // Data for each particle
    vertex Vertex {
        pos: [f32; 2] = "a_Pos",
        radius: f32 = "a_Radius",
        time: f32 = "a_Time",
        base_color: [f32; 4] = "a_BaseColor",
        new_color: [f32; 4] = "a_NewColor",
    }

    // Aspect ratio to keep particles round
    constant Locals {
        transformation: [[f32; 4]; 4] = "u_Transformation",
    }

    // Particle render pipeline
    pipeline circles {
        vbuf: gfx::VertexBuffer<Vertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out_color: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}

impl Vertex {
    // Construct new particles far away so they can't be seen initially
    fn new() -> Vertex {
        Vertex {
            pos: [std::f32::INFINITY, std::f32::INFINITY],
            radius: 50.0,
            time: 0.0,
            base_color: Default::default(),
            new_color: Default::default(),
        }
    }
}

struct Camera {
    pub pos: Vector2<f32>,
    pub up: bool,
    pub right: bool,
    pub down: bool,
    pub left: bool,
}

impl Camera {
    pub fn compute(&mut self, time: f32) {
        self.pos += self.camera_speed() * time;
    }

    fn camera_speed(&self) -> Vector2<f32> {
        let angle = match (self.up, self.right, self.down, self.left) {
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

struct App<R: gfx::Resources> {
    bundle: Bundle<R, circles::Data<R>>,
    circle: Vertex,
    time_start: Instant,
    camera: Camera,
}

fn create_shader_set<R: gfx::Resources, F: gfx::Factory<R>>(factory: &mut F,
                                                            vs_code: &[u8],
                                                            gs_code: &[u8],
                                                            ps_code: &[u8]) -> ShaderSet<R> {
    let vs = factory.create_shader_vertex(vs_code).expect("Failed to compile vertex shader");
    let gs = factory.create_shader_geometry(gs_code).expect("Failed to compile geometry shader");
    let ps = factory.create_shader_pixel(ps_code).expect("Failed to compile pixel shader");
    ShaderSet::Geometry(vs, gs, ps)
}

impl<R: gfx::Resources> gfx_app::Application<R> for App<R> {
    fn new<F: gfx::Factory<R>>(factory: &mut F, backend: gfx_app::shade::Backend,
                               window_targets: gfx_app::WindowTargets<R>) -> Self {
        use gfx::traits::FactoryExt;

        // Load in our vertex, geometry and pixel shaders
        let vs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shaders/circle_150.glslv"),
            .. gfx_app::shade::Source::empty()
        };
        let gs = gfx_app::shade::Source {
            glsl_150: include_bytes!("shaders/circle_150.glslg"),
            .. gfx_app::shade::Source::empty()
        };
        let ps = gfx_app::shade::Source {
            glsl_150: include_bytes!("shaders/circle_150.glslf"),
            .. gfx_app::shade::Source::empty()
        };

        let shader_set = create_shader_set(
            factory,
            vs.select(backend).unwrap(),
            gs.select(backend).unwrap(),
            ps.select(backend).unwrap(),
        );

        // Create 4096 particles, using one point vertex per particle
        let mut circle = Vertex::new();

        // Create a dynamic vertex buffer to hold the particle data
        let vbuf = factory.create_buffer(1,
                                         buffer::Role::Vertex,
                                         gfx::memory::Usage::Dynamic,
                                         Bind::empty())
            .expect("Failed to create vertex buffer");
        let slice = Slice::new_match_vertex_buffer(&vbuf);

        // Construct our pipeline state
        let pso = factory.create_pipeline_state(
            &shader_set,
            Primitive::PointList,
            Rasterizer::new_fill(),
            circles::new()
        ).unwrap();

        circle.pos = [0.0, 0.0];
        circle.base_color = [rand::random(), rand::random(), rand::random(), 1.0];
        circle.new_color = [rand::random(), rand::random(), rand::random(), 1.0];

        let data = circles::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out_color: window_targets.color,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            circle: circle,
            time_start: Instant::now(),
            camera: Camera {
                pos: Vector2::zero(),
                up: false,
                right: false,
                down: false,
                left: false,
            }
        }
    }

    fn render<C: gfx::CommandBuffer<R>>(&mut self, encoder: &mut gfx::Encoder<R, C>) {
        // Compute the time since last frame
        let delta = self.time_start.elapsed();
        self.time_start = Instant::now();
        let delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 / 1000_000_000.0;

        let anim_duration = 1.7;
        self.circle.time = self.circle.time + delta / anim_duration;
        if self.circle.time > 1. {
            self.circle.time %= 1.;
            let base_color = self.circle.base_color;
            self.circle.base_color = self.circle.new_color;
            self.circle.new_color = base_color;
        }

        let (width, height, _, _) = self.bundle.data.out_color.get_dimensions();

        self.camera.compute(delta);
        let scale_vector = Vector3::new(1.0 / width as f32 * 4.0, 1.0 / height as f32 * 4.0, 1.0);
        let camera_translation = Matrix4::from_translation(
            self.camera.pos.neg().extend(0.0).mul_element_wise(scale_vector)
        );
        let scale = Matrix4::from_nonuniform_scale(scale_vector.x, scale_vector.y, scale_vector.z);

        // Pass in the aspect ratio to the geometry shader
        let locals = Locals { transformation: (camera_translation * scale).into() };
        encoder.update_constant_buffer(&self.bundle.data.locals, &locals);
        // Update the vertex data with the changes to the particles array
        encoder.update_buffer(&self.bundle.data.vbuf, &[self.circle], 0).unwrap();
        // Clear the background to dark blue
        encoder.clear(&self.bundle.data.out_color, [0.1, 0.2, 0.3, 1.0]);
        // Draw the particles!
        self.bundle.encode(encoder);
    }

    fn on_resize(&mut self, window_targets: gfx_app::WindowTargets<R>) {
        self.bundle.data.out_color = window_targets.color;
    }

    fn on(&mut self, event: winit::WindowEvent) {
        match event {
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::W), _) => {
                self.camera.up = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::D), _) => {
                self.camera.right = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::S), _) => {
                self.camera.down = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Pressed, _, Some(winit::VirtualKeyCode::A), _) => {
                self.camera.left = true;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::W), _) => {
                self.camera.up = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::D), _) => {
                self.camera.right = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::S), _) => {
                self.camera.down = false;
            },
            winit::WindowEvent::KeyboardInput(winit::ElementState::Released, _, Some(winit::VirtualKeyCode::A), _) => {
                self.camera.left = false;
            },
            _ => ()
        }
    }
}

pub fn main() {
    use gfx_app::Application;
    App::launch_simple("circles-rs");
}
