use gfx;
use gfx::format::{Srgba8, Rgba16F};

use ::game::entities::circle::*;
pub type ColorFormat = Srgba8;
pub type DepthFormat = gfx::format::DepthStencil;

gfx_defines! {
    constant Locals {
        transformation: [[f32; 4]; 4] = "u_Transformation",
    }

    pipeline Circles {
        vbuf: gfx::VertexBuffer<Circle> = (),
        locals: gfx::ConstantBuffer<Locals> = "Locals",
        out: gfx::BlendTarget<ColorFormat> = ("Target0", gfx::state::ColorMask::all(), gfx::preset::blend::ALPHA),
    }
}
