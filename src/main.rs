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
        // ball.pos = mouse_position();
        // ball.rotation += 0.05;
        // if screen_width() != width {ft = ui::ft();}
        width = screen_width();
        let height = screen_height();

        // points.push(pos);
        // let mut prev = points[0];
        // for point in &points {
        //     draw_line(point.0, point.1, prev.0, prev.1,  3.0, BLACK);
        //     prev = *point;
        // }
        let path = ui.path.to_vec();
        if path.len() > 0 {
            let mut prev = path[0];
            for point in &path {
                draw_line(point.0, point.1, prev.0, prev.1,  3.0, RED);
                prev = *point;
            }
        }

        let robot = Arc::clone(&robot);

        {
            robot.lock().unwrap().render();
            pos = robot.lock().unwrap().position;
        }
        // thread::sleep(Duration::from_millis(10));
        
        match ui.state() {
            ui::State::Create(_) => {},
            ui::State::Driver => {
                driver::drive(&mut vel, &mut robot.lock().unwrap());
                autonStarted = false;
            },
            ui::State::Auton => {
                if !autonStarted {
                    autonStarted = true;
                    // thread::spawn(move || {
                    //     // movement::pidMTP(&robot, (width / 2.0, height / 2.0), 115.0, 800, lCons, rCons, 10.0);
                    //     // movement::boomerang(&robot, (width - width / 8.0, height - height/ 6.0), 40000, 0.7, PI/2.0, 90.0, lCons, rCons, 0.0);
                    //     // movement::boomerang(&robot, (width/2.0, height/2.0 - 330.0), 5000, 0.7, PI/2.0, 90.0, lCons, rCons, 0.0);
                    //     // movement::eulerTurn(&robot, 0.0, -0.0002, movement::eulerTurn(&robot, 45.0, 0.0002, 0.0, 3000, 1, turnCons), 3000, -1, turnCons);
                    //     // let path = vec![(398.32422, 153.41406), (559.21094, 99.73828), (810.08594, 135.30078), (932.33984, 164.47266), (1003.5625, 239.64063), (991.8906, 394.1797), (903.0703, 482.3047), (853.76953, 604.375), (726.21094, 601.0117), (516.59375, 587.25), (504.28125, 469.16797)];

                    //     // movement::followPath(&robot, path, 100000, 0.7, PI/2.0, 90.0, lCons, rCons, 0.0);
                    //     movement::moveToPurePursuit(&robot, path, 80.0, 2, 1000);
                    // });
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