use std::f32::consts::PI;

use crate::{util::{self, draw_arc, draw_arc_lines}, hex, ui::{self, ft}};
use macroquad::prelude::*;

const R303: f32 = 0.57735026919;
const R306: f32 = 0.28867513459;

pub struct Triball {
    pub size: f32,
    pub pos: (f32, f32),
    pub rotation: f32,
    pub vel: Vec3,
    pub accel: Vec3
}

pub struct Field {
    size: f32,
    pos: (f32, f32), //top left corner
    triballs: Vec<Triball>,
}

impl Triball {
    pub fn render(&self) {
        let v1 = util::rotate![0.0, self.size * R303, self.rotation, self.pos]; 
        let v2 = util::rotate![-self.size / 2.0,-self.size * R306, self.rotation, self.pos];
        let v3 = util::rotate![self.size / 2.0, -self.size * R306, self.rotation, self.pos];
        draw_arc(v1, self.size, self.rotation - PI / 3.0, self.rotation - 2.0 * PI/3.0, GREEN);
        draw_arc(v2, self.size, self.rotation, self.rotation + PI / 3.0, GREEN);
        draw_arc(v3, self.size, self.rotation + 2.0 * PI/3.0, self.rotation + 3.0 * PI/3.0, GREEN);
        // draw_arc_lines(v1, self.size, self.rotation - PI / 3.0, self.rotation - 2.0 * PI/3.0, 1.0, hex!(0xDDDDDF));
        // draw_arc_lines(v2, self.size, self.rotation, self.rotation + PI / 3.0, 1.0, hex!(0xDDDDDF));
        // draw_arc_lines(v3, self.size, self.rotation + 2.0 * PI/3.0, self.rotation + 3.0 * PI/3.0, 1.0, hex!(0xDDDDDF));

        // draw_circle(v1.x, v1.y, 5.0, RED);
        // draw_circle(v2.x, v2.y, 5.0, GREEN);
        // draw_circle(v3.x, v3.y, 5.0, BLUE);
    }

    pub fn physics(&mut self) {
        // self.vel += self.accel;
        // self.pos.0 += self.vel.x;
        // self.pos.1 += self.vel.y;
        // self.rotation += self.vel.z;

        // self.vel.x -=  0.01 * self.vel.x.signum();
        // self.vel.y -=  0.01 * self.vel.y.signum();
        // self.vel.z -= 0.005 * self.vel.z.signum();
    }
}

impl Field {
    pub fn new() -> Field{
        let size = screen_width().min(screen_height()) * 0.8;
        Field { size, pos: (screen_width() / 2.0 - size / 2.0, screen_height() / 2.0 - size / 2.0), triballs: Vec::new()}
    }

    pub fn render(&mut self) {
        let size = ft() * 12.0 * 0.9;
        self.size = size;
        self.pos = (screen_width() / 2.0 - size / 2.0, screen_height() / 2.0 - size / 2.0);

        let tileSize = self.size / 6.0;
        let colors = [hex!(0xDCABDF), hex!(0xC792DF)];
        for i in 0..6 {
            for j in 0..6 {
                let index = (i+j) % 2;
                draw_rectangle(self.pos.0 + i as f32 * tileSize, self.pos.1 + j as f32 * tileSize, tileSize, tileSize, colors[index]);
            }
        }
        draw_rectangle_lines(self.pos.0, self.pos.1, self.size, self.size, 6.0, hex!(0xD0C4DF));
    }

    // pub fn update(&mut self) {
    //     let size = screen_width().min(screen_height()) * 0.9;
    //     self.size = size;
    //     self.pos = (screen_width() / 2.0 - size / 2.0, screen_height() / 2.0 - size / 2.0);
    // }
}
