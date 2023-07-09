#![allow(dead_code)]
#![allow(non_snake_case)]

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
mod robot;
mod util;
mod ui;
mod movement;
mod driver;
use std::thread;


#[macroquad::main("Events")]
async fn main() {
    let robot = Arc::new(Mutex::new(robot::Robot {
        position: (100.0, 100.0),
        heading: 0.0,
    }));
    let mut vel : (f32, f32) = (0.0,0.0);
    let mut ui = ui::Ui::new();
    let mut autonStarted = false;

    let lCons = util::PidConstants{
        p: 0.03,
        i: 0.0,
        d: 0.0,
        tolerance: 0.0,
        integralThreshold: 0.0,
        maxIntegral: 0.0
    };

    let rCons = util::PidConstants{
        p: 0.015,
        i: 0.0,
        d: 0.0,
        tolerance: 0.0,
        integralThreshold: 0.0,
        maxIntegral: 0.0
    };

    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut pos: (f32, f32) = (0.0,0.0);
    loop {
        clear_background(WHITE);
        let robot = Arc::clone(&robot);

        {
            robot.lock().unwrap().render();
            pos = robot.lock().unwrap().position;
        }

        thread::sleep(Duration::from_millis(10));
        points.push(pos);

        let mut prev = points[0];
        for point in &points {
            draw_line(point.0, point.1, prev.0, prev.1,  3.0, BLACK);
            prev = *point;
        }

        if ui.state() == ui::State::Driver {
            driver::drive(&mut vel, &mut robot.lock().unwrap());
            autonStarted = false;
        }

        if ui.state() == ui::State::Auton {
            if !autonStarted {
                autonStarted = true;
                thread::spawn(move || {
                    movement::pidMTP(robot, (screen_width() / 2.0, screen_height() / 2.0), 0.0, 2000, lCons, rCons);
                });
            }
        }
        ui.render();
        next_frame().await; 
    }
}