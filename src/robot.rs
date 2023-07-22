use macroquad::{prelude::*, color};
use std::f32::consts::PI;
use crate::glam::{vec2, vec3, vec4, Mat4, Vec2};
use crate::ui::ft;
use crate::util;
// pub const self.robotSize: f32 = 30.0;

pub struct Robot {
    pub position: (f32, f32),
    pub heading: f32,
    pub robotSize: f32
}

impl Robot {
    pub fn render(&mut self){
        self.robotSize = ft() / 12.0 * 15.0 / 2.0;
        let v1 = util::rotate![-self.robotSize, self.robotSize, self.heading, self.position];
        let v2 = util::rotate![self.robotSize, self.robotSize, self.heading, self.position];
        let v3 = util::rotate![-self.robotSize, -self.robotSize, self.heading, self.position];
        let v4 = util::rotate![self.robotSize, -self.robotSize, self.heading, self.position];
        draw_triangle(v1, v2, v3, Color::from_hex(0x6F2232));
        draw_triangle(v3, v4, v2, Color::from_hex(0x6F2232));
        draw_line(v3.x, v3.y, v4.x, v4.y, 4.0, Color::from_hex(0x950740));
    }

    pub fn step(&mut self, d: (f32, f32)) {
        let deltaRotation = (d.1 - d.0) / (self.robotSize);
        self.heading += deltaRotation;
        if deltaRotation == 0.0 {
            self.position.0  -= (PI/2.0 + self.heading).cos() * d.1;
            self.position.1 -= (PI/2.0 + self.heading).sin() * d.1;
        }
        else {
            let r = d.0 / deltaRotation + self.robotSize / 2.0;
            let relativeY = 2.0 * (deltaRotation / 2.0).sin() * r;
            let rotationOffset = self.heading + (deltaRotation / 2.0);
            let theta = PI / 2.0 + rotationOffset;
            let radius = relativeY;
            self.position.0 -= radius * theta.cos();
            self.position.1 -= radius * theta.sin();
        }
    }
}   
