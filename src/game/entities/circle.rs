use std;

gfx_defines! {
    vertex Circle {
        pos: [f32; 2] = "a_Pos",
        radius: f32 = "a_Radius",
        time: f32 = "a_Time",
        base_color: [f32; 4] = "a_BaseColor",
        new_color: [f32; 4] = "a_NewColor",
    }
}

impl Circle {
    // Construct new particles far away so they can't be seen initially
    pub fn new() -> Circle {
        Circle {
            pos: [std::f32::INFINITY, std::f32::INFINITY],
            radius: 50.0,
            time: 0.0,
            base_color: Default::default(),
            new_color: Default::default(),
        }
    }
}
