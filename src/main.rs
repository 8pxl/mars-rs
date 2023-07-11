#![allow(dead_code)]
#![allow(non_snake_case)]

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
mod robot;
mod util;
mod ui;
mod paths;
mod movement;
mod driver;
// mod skin;
use std::thread;


#[macroquad::main("Events")]
async fn main() {
    let editbox_style = root_ui().style_builder()
    .color(Color::from_hex(0x1F182F))
    .build();

    let window_style = root_ui().style_builder()
    .color(Color::from_hex(0x335c67))
    .build();

    let label_style = root_ui().style_builder()
    .text_color(Color::from_hex(0x6F2232))
    .build();

    let button_style = root_ui().style_builder()
    .color(hex!(0x221D2F))
    .text_color(Color::from_hex(0x6F2232))
    .color_hovered(hex!(0x332B45))
    .build();

    let skin = Skin {
        editbox_style,
        window_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    };

    root_ui().push_skin(&skin); 

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

    // let lCons = util::PidConstants{
    //     p: 0.0003,
    //     i: 0.0,
    //     d: 0.0,
    //     tolerance: 0.0,
    //     integralThreshold: 0.0,
    //     maxIntegral: 0.0
    // };

    // let rCons = util::PidConstants{
    //     p: 0.00015,
    //     i: 0.0,
    //     d: 0.0,
    //     tolerance: 0.0,
    //     integralThreshold: 0.0,
    //     maxIntegral: 0.0
    // };

    let mut points: Vec<(f32, f32)> = Vec::new();
    let mut pos: (f32, f32) = robot.lock().unwrap().position;
    loop {
        let height = screen_height();
        let width = screen_width();
        clear_background(Color::from_hex(0x1A1A1D));

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
                        // movement::pidMTP(&robot, (width / 2.0, height / 2.0), 115.0, 800, lCons, rCons);
                        movement::boomerang(&robot, (width - width / 8.0, height - height/ 6.0), 40000, 0.7, PI/2.0, 90.0, lCons, rCons);
                        // movement::followPath(&robot, path, 10000, 0.7, PI/2.0, 90.0, lCons, rCons);
                    });
                }
            }
        }
        ui.render();
        next_frame().await; 
    }
}