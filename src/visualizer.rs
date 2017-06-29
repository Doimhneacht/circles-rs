use gfx;
use gfx::{Bundle, ShaderSet, Primitive, buffer, Bind, Slice};
use gfx::state::Rasterizer;
use gfx::traits::FactoryExt;
use cgmath::{Matrix4, Vector3, ElementWise};
use std::ops::Neg;
use std::vec::Vec;

use pipeline::*;

type RenderTarget<R> = gfx::handle::RenderTargetView<R, ColorFormat>;
type DepthTarget<R> = gfx::handle::DepthStencilView<R, DepthFormat>;

const CLEAR_COLOR: [f32; 4] = [0.1, 0.2, 0.3, 1.0];

pub struct Visualizer<R, F>
    where R: gfx::Resources, F: gfx::Factory<R>
{
    circles: Bundle<R, circles_pipeline::Data<R>>,
    main_depth: gfx::handle::DepthStencilView<R, DepthFormat>,
    factory: F,
}

gfx_defines! {
    vertex CircleVertex {
        position: [f32; 2] = "a_Position",
        radius: f32 = "a_Radius",
        time: f32 = "a_Time",
        base_color: [f32; 4] = "a_BaseColor",
        new_color: [f32; 4] = "a_NewColor",
    }
}

impl<R, F> Visualizer<R, F>
    where R: gfx::Resources, F: gfx::Factory<R>
{
    pub fn new(mut factory: F,
                  main_color: RenderTarget<R>,
                  main_depth: DepthTarget<R>)
                  -> Visualizer<R, F>
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
                circles_pipeline::new()
            ).unwrap();

            let vbuf = factory.create_buffer(0,
                                             buffer::Role::Vertex,
                                             gfx::memory::Usage::Dynamic,
                                             Bind::empty())
                .expect("Failed to create vertex buffer");

            let slice = Slice::new_match_vertex_buffer(&vbuf);

            let data = circles_pipeline::Data {
                vbuf: vbuf,
                locals: factory.create_constant_buffer(1),
                out: main_color,
            };

            Bundle::new(slice, pso, data)
        };

        Visualizer {
            circles: circles,
            main_depth: main_depth,
            factory: factory,
        }
    }

    pub fn render<C>(&mut self, encoder: &mut gfx::Encoder<R, C>,
                     player: &::game::entities::Player,
                     food: &Vec<::game::entities::Food>)
        where C: gfx::CommandBuffer<R>
    {
        let (width, height, _, _) = self.circles.data.out.get_dimensions();

        let scale_vector = Vector3::new(1.0 / width as f32 * 4.0, 1.0 / height as f32 * 4.0, 1.0);
        let camera_translation = Matrix4::from_translation(
            player.circle.position.neg().extend(0.0).mul_element_wise(scale_vector)
        );
        let scale = Matrix4::from_nonuniform_scale(scale_vector.x, scale_vector.y, scale_vector.z);

        let locals = Locals { transformation: (camera_translation * scale).into() };
        encoder.update_constant_buffer(&self.circles.data.locals, &locals);

        let mut entities: Vec<CircleVertex> = Vec::new();
        entities.extend( food.iter().map(|ref entity| circle_to_vertex(&entity.circle)) );
        entities.push(circle_to_vertex(&player.circle));

        self.circles.data.vbuf = self.factory.create_buffer(entities.len(),
                                                       buffer::Role::Vertex,
                                                       gfx::memory::Usage::Dynamic,
                                                       Bind::empty())
            .expect("Failed to create vertex buffer");
        self.circles.slice = Slice::new_match_vertex_buffer(&self.circles.data.vbuf);

        encoder.update_buffer(&self.circles.data.vbuf, &entities, 0).unwrap();

        encoder.clear(&self.circles.data.out, CLEAR_COLOR);
        self.circles.encode(encoder);
    }

    pub fn targets(&mut self) -> (&mut RenderTarget<R>, &mut DepthTarget<R>) {
        (&mut self.circles.data.out, &mut self.main_depth)
    }
}

fn circle_to_vertex(circle: &::game::entities::components::Circle) -> CircleVertex {
    CircleVertex {
        position: circle.position.into(),
        radius: circle.radius,
        time: circle.time,
        base_color: circle.base_color.into(),
        new_color: circle.new_color.into(),
    }
}
