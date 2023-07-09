#![allow(dead_code)]
#![allow(non_snake_case)]

use macroquad::prelude::*;
use std::f32::consts::PI;
mod robot;
mod util;
const MAX_VEL: f32 = 4.0;

#[macroquad::main("Events")]
async fn main() {
    let mut robot = robot::Robot {
        position: (100.0, 100.0),
        heading: 0.0,
    };
    let mut vel : (f32, f32) = (0.0,0.0);
    loop {
        clear_background(WHITE);
        robot.render();
        println!("{:?}", robot.position);

        // if is_key_down(KeyCode::W) { 
        //     robot.position.1 -= 1.0;
        // }
        // if is_key_down(KeyCode::A) { 
        //     robot.position.0 -= 1.0;
        // }
        // if is_key_down(KeyCode::S) { 
        //     robot.position.1 += 1.0;
        // }
        // if is_key_down(KeyCode::D) { 
        //     robot.position.0 += 1.0;
        // }
        // if is_key_down(KeyCode::L) { 
        //     robot.heading += 0.05;
        // }
        // if is_key_down(KeyCode::K) { 
        //     robot.heading -= 0.05;
        // }

        let absv = vel.0.abs();
        let absv1 = vel.1.abs();
        let sgnv = vel.0.signum();
        let sgnv1 = vel.1.signum();

        if absv > MAX_VEL {
            vel.0 = MAX_VEL * sgnv
        }
        if absv1 > MAX_VEL {
            vel.1 = MAX_VEL * sgnv1
        }

        vel.0 = if absv > 0.07 {vel.0 - sgnv * 0.07} else {0.0};
        vel.1 = if absv1 > 0.07 {vel.1 - sgnv1 * 0.07} else {0.0};

        robot.step(vel);
        next_frame().await; 

        if is_key_down(KeyCode::W) { 
            vel.0 += 0.2;
            vel.1 += 0.2;
        }
        if is_key_down(KeyCode::A) { 
            vel.1 += 0.2 * -sgnv1;
        }
        if is_key_down(KeyCode::S) { 
            vel.0 -= 0.2;
            vel.1 -= 0.2;
        }
        if is_key_down(KeyCode::D) { 
            vel.0 += 0.2 * -sgnv;
        }
    }
}