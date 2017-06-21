use gfx;
use gfx::format::Srgba8;

use super::visualizer::CircleVertex;
pub type ColorFormat = Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    constant Locals {
        transformation: [[f32; 4]; 4] = "u_Transformation",
    }

    pipeline circles_pipeline {
        vbuf: gfx::VertexBuffer<CircleVertex> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}
