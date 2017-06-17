// Copyright 2014 The Gfx-rs Developers.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

extern crate rand;
extern crate cgmath;
#[macro_use]
extern crate gfx;
extern crate gfx_app;

use std::time::Instant;

use gfx_app::ColorFormat;
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;
use cgmath::{Matrix4, Vector3, ElementWise};

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

struct App<R: gfx::Resources> {
    bundle: Bundle<R, circles::Data<R>>,
    circle: Vertex,
    transformation: [[f32; 4]; 4],
    time_start: Instant,
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

        let (width, height, _, _) = window_targets.color.get_dimensions();

        let scale_vector = Vector3::new(1.0 / width as f32 * 4.0, 1.0 / height as f32 * 4.0, 1.0);
        let camera_translation = Matrix4::from_translation(Vector3::new(-100.0, -50.0, 0.0).mul_element_wise(scale_vector));
        let scale = Matrix4::from_nonuniform_scale(scale_vector.x, scale_vector.y, scale_vector.z);

        let data = circles::Data {
            vbuf: vbuf,
            locals: factory.create_constant_buffer(1),
            out_color: window_targets.color,
        };

        App {
            bundle: Bundle::new(slice, pso, data),
            circle: circle,
            transformation: (camera_translation * scale).into(),
            time_start: Instant::now(),
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

        // Pass in the aspect ratio to the geometry shader
        let locals = Locals { transformation: self.transformation };
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
}

pub fn main() {
    use gfx_app::Application;
    App::launch_simple("circles-rs");
}
