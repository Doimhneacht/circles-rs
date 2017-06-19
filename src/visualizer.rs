use gfx;
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;
use gfx::traits::FactoryExt;
use cgmath::{Matrix4, Vector3, ElementWise};
use std::ops::Neg;

use rand;

use ::game::entities::*;
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

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>,
                     camera: &::game::Camera,
                     circle: &::game::entities::Circle)
        where C: gfx::CommandBuffer<R>
    {
        let (width, height, _, _) = self.circles.data.out.get_dimensions();

        let scale_vector = Vector3::new(1.0 / width as f32 * 4.0, 1.0 / height as f32 * 4.0, 1.0);
        let camera_translation = Matrix4::from_translation(
            camera.position().neg().extend(0.0).mul_element_wise(scale_vector)
        );
        let scale = Matrix4::from_nonuniform_scale(scale_vector.x, scale_vector.y, scale_vector.z);

         // Pass in the aspect ratio to the geometry shader
        let locals = Locals { transformation: (camera_translation * scale).into() };
        encoder.update_constant_buffer(&self.circles.data.locals, &locals);
        // Update the vertex data with the changes to the particles array
        encoder.update_buffer(&self.circles.data.vbuf, &[*circle], 0).unwrap();

        encoder.clear(&self.circles.data.out, CLEAR_COLOR);
        self.circles.encode(encoder);
    }
}
