use std;
use rand;

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
            pos: [0.0, 0.0],
            radius: 50.0,
            time: 0.0,
            base_color: [rand::random(), rand::random(), rand::random(), 1.0],
            new_color: [rand::random(), rand::random(), rand::random(), 1.0],
        }
    }

    pub fn swap_colors(&mut self) {
        let temp = self.base_color;
        self.base_color = self.new_color;
        self.new_color = temp;
    }
}
