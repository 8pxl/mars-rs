#![allow(dead_code)]
#![allow(non_snake_case)]

use macroquad::prelude::*;
// use robot::ROBOT_SIZE;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
mod robot;
mod util;
mod ui;
mod paths;
mod movement;
mod driver;
mod field;
// mod skin;
use std::thread;


#[macroquad::main("Events")]
async fn main() {
    ui::setSkin();
    let mut autonStarted = false;
    let mut points: Vec<(f32, f32)> = Vec::new();

    // let mut ft = ui::ft();
    // let unitPtr = &ft;

    let mut ball = field::Triball {
        size: 30.0, 
        pos: (400.0, 200.0), 
        rotation: PI / 3.0,
        vel: vec3(0.2,0.2,0.05),
        accel: vec3(0.0,0.0,0.0),
    };

    let mut field = field::Field::new();
    let mut ui = ui::Ui::new();
    let robot = Arc::new(Mutex::new(robot::Robot {
        position: (screen_width()/2.0, screen_height()/3.8),
        heading: 0.0,
        robotSize: 0.0
    }));
    
    let mut pos = robot.lock().unwrap().position;
    let mut vel : (f32, f32) = (0.0,0.0);
    let mut width;
    loop {
        clear_background(Color::from_hex(0x1A1A1D));
        field.render();
        ball.render();
        ball.physics();
        // ball.rotation += 0.05;
        // if screen_width() != width {ft = ui::ft();}
        width = screen_width();
        let height = screen_height();

        points.push(pos);
        let mut prev = points[0];
        for point in &points {
            draw_line(point.0, point.1, prev.0, prev.1,  3.0, BLACK);
            prev = *point;
        }
        // let mut prev = path[0];
        // for point in &path {
        //     draw_line(point.0, point.1, prev.0, prev.1,  3.0, RED);
        //     prev = *point;
        // }

        let robot = Arc::clone(&robot);
        {
            robot.lock().unwrap().render();
            pos = robot.lock().unwrap().position;
        }
        thread::sleep(Duration::from_millis(10));
        
        match ui.state() {
            ui::State::Create(_) => {},
            ui::State::Driver => {
                driver::drive(&mut vel, &mut robot.lock().unwrap());
                autonStarted = false;
            },
            ui::State::Auton => {
                if !autonStarted {
                    autonStarted = true;
                    thread::spawn(move || {
                        // movement::pidMTP(&robot, (width / 2.0, height / 2.0), 115.0, 800, lCons, rCons, 10.0);
                        // movement::boomerang(&robot, (width - width / 8.0, height - height/ 6.0), 40000, 0.7, PI/2.0, 90.0, lCons, rCons, 0.0);
                        movement::boomerang(&robot, (width/2.0, height/2.0), 5000, 0.7, PI/2.0, 90.0, lCons, rCons, 0.0);
                        // movement::eulerTurn(&robot, 0.0, -0.0002, movement::eulerTurn(&robot, 45.0, 0.0002, 0.0, 3000, 1, turnCons), 3000, -1, turnCons);
                        // let path = vec![(94.38672, 101.21875), (316.25, 367.63672), (416.45703, 520.4297), (645.03516, 522.4297), (647.09375, 638.75), (966.8867, 491.9961), (1229.8438, 331.79297), (1218.7109, 191.0039), (1033.≥0742, 147.9336), (860.5, 277.73047), (845.53125, 110.87891), (623.3711, 123.828125), (576.3125, 297.67578), (497.1836, 187.41016), (384.39063, 123.94141), (304.07422, 156.35938), (258.46875, 81.24609)];
                        // movement::followPath(&robot, path, 100000, 0.7, PI/2.0, 90.0, lCons, rCons);
                    });
                }
            }
        }
        // util::draw_arc(199.0, 199.0, 80.0, 0.0, PI, RED);
        ui.render();
        next_frame().await; 
    }
}

const lCons: util::PidConstants = util::PidConstants{
    p: 0.03,
    i: 0.0,
    d: 0.0,
    tolerance: 0.0,
    integralThreshold: 0.0,
    maxIntegral: 0.0
};

const rCons: util::PidConstants = util::PidConstants{
    p: 0.015,
    i: 0.0,
    d: 0.0,
    tolerance: 0.0,
    integralThreshold: 0.0,
    maxIntegral: 0.0
};

const turnCons: util::PidConstants = util::PidConstants{
    p: 0.024,
    i: 0.0,
    d: 0.0,
    tolerance: 0.0,
    integralThreshold: 0.0,
    maxIntegral: 0.0
};