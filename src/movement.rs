use macroquad::prelude::mouse_position;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_line;

use crate::robot;
use crate::util;
use crate::util::absoluteAngleToPoint;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

//https://www.desmos.com/calculator/vhte05bsot for scaling
pub fn pidMTPVel(position: (f32, f32), heading: f32, target: (f32, f32), rotationCut: f32,  lCont: &mut util::Pid, rCont: &mut util::Pid) -> (f32, f32) {
    let linearError = util::dist(position,target);
    let targetHeading = util::absoluteAngleToPoint(position, target);
    let rotationError = util::minError(targetHeading, if heading < 0.0 {360.0 + heading} else {heading});
    let scale = 90.0 / rotationCut;
    let cre = if rotationError.abs() > rotationCut {0.1} else {(scale * rotationError.to_radians()).cos()};
    let angularVel = rCont.out(rotationError);
    let linearVel = cre * lCont.out(linearError);
    let rVel = linearVel - angularVel;
    let lVel = linearVel + angularVel;
    (lVel, rVel)
}

pub fn pidMTP(robot: &Arc<Mutex<robot::Robot>>, target: (f32, f32), rotationCut: f32, timeout: u16, lConstants: util::PidConstants, rConstants: util::PidConstants) {
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);
    while start.elapsed().as_millis() < timeout.into() {
        {
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let heading = robot.heading.to_degrees() % 360.0;
            robot.step(pidMTPVel(pos, heading, target, rotationCut, &mut lCont, &mut rCont));
        }

        thread::sleep(Duration::from_millis(10));
    }
}

pub fn boomerang(robot: &Arc<Mutex<robot::Robot>>, target: (f32, f32), timeout: u16, dLead: f32, thetaEnd: f32, rotationCut: f32, lConstants: util::PidConstants, rConstants: util::PidConstants)
{
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);

    while start.elapsed().as_millis() < timeout.into() {
        {
            let target = mouse_position();
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let heading = robot.heading.to_degrees() % 360.0;
            let h = (pos.0 - target.0).hypot(pos.1 - target.1);
            let carrot = (target.0 - (h * thetaEnd.sin() * dLead), target.1 - (h * thetaEnd.cos() * dLead));
            use macroquad::prelude::RED;
            use macroquad::prelude::BLACK;
            draw_line(pos.0,pos.1,carrot.0, carrot.1, 2.0, RED);
            draw_circle(target.0, target.1, 1.0, BLACK);
            robot.step(pidMTPVel(pos, heading, carrot, rotationCut, &mut lCont, &mut rCont));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn followPath(robot: &Arc<Mutex<robot::Robot>>, path: Vec<(f32,f32)>, timeout: u16, dLead: f32, thetaEnd: f32, rotationCut: f32, lConstants: util::PidConstants, rConstants: util::PidConstants) {
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);
    let mut pathIndex = 0;

    while start.elapsed().as_millis() < timeout.into() {
        {
            let target = path[pathIndex];
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let thetaEnd = absoluteAngleToPoint(pos, target).to_radians();
            let heading = robot.heading.to_degrees() % 360.0;
            let h = (pos.0 - target.0).hypot(pos.1 - target.1);
            let carrot = (target.0 - (h * thetaEnd.sin() * dLead), target.1 - (h * thetaEnd.cos() * dLead));
            use macroquad::prelude::RED;
            use macroquad::prelude::BLACK;
            draw_line(pos.0,pos.1,carrot.0, carrot.1, 2.0, RED);
            draw_circle(target.0, target.1, 1.0, BLACK);
            robot.step(pidMTPVel(pos, heading, carrot, rotationCut, &mut lCont, &mut rCont));
            if util::dist(pos, target) < 40.0 {
                if pathIndex < path.len() {pathIndex += 1};
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}