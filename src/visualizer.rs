use gfx;
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;
use gfx::traits::FactoryExt;

use rand;

use ::game::entities::circle::*;
use pipeline::*;

const CLEAR_COLOR: [f32; 4] = [1.0, 1.0, 1.0, 1.0];

pub struct Visualizer<R>
    where R: gfx::Resources
{
    circles: Bundle<R, Circles::Data<R>>,
}

impl<R> Visualizer<R>
    where R: gfx::Resources
{
    pub fn new<F>(mut factory: F,
                  main_color: gfx::handle::RenderTargetView<R, ColorFormat>,
                  _main_depth: gfx::handle::DepthStencilView<R, DepthFormat>)
                  -> Visualizer<R>
        where F: gfx::Factory<R>
    {
        let circles = {
            let shader_set = ShaderSet::Geometry(
                factory.create_shader_vertex(include_bytes!("shaders/circle_150.glslv"))
                    .expect("Failed to compile vertex shader"),
                factory.create_shader_geometry(include_bytes!("shaders/circle_150.glslg"))
                    .expect("Failed to compile geometry shader"),
                factory.create_shader_pixel(include_bytes!("shaders/circle_150.glslf"))
                    .expect("Failed to compile fragment shader")
            );

            let pso = factory.create_pipeline_state(
                &shader_set,
                Primitive::PointList,
                Rasterizer::new_fill(),
                Circles::new()
            ).unwrap();

            let mut circle = Circle::new();
            circle.pos = [0.0, 0.0];
            circle.base_color = [rand::random(), rand::random(), rand::random(), 1.0];
            circle.new_color = [rand::random(), rand::random(), rand::random(), 1.0];

            let vbuf = factory.create_buffer(1,
                                             buffer::Role::Vertex,
                                             gfx::memory::Usage::Dynamic,
                                             Bind::empty())
                .expect("Failed to create vertex buffer");
            let slice = Slice::new_match_vertex_buffer(&vbuf);

            let data = Circles::Data {
                vbuf: vbuf,
                locals: factory.create_constant_buffer(1),
                out: main_color,
            };

            Bundle::new(slice, pso, data)
        };

        Visualizer {
            circles: circles,
        }
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>)
        where C: gfx::CommandBuffer<R>
    {
        encoder.clear(&self.circles.data.out, CLEAR_COLOR);
        self.circles.encode(encoder);
    }
}
