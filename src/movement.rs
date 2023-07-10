use macroquad::shapes::draw_circle;

use crate::robot;
use crate::util;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};


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

pub fn pidMTP(robot: Arc<Mutex<robot::Robot>>, target: (f32, f32), rotationCut: f32, timeout: u16, lConstants: util::PidConstants, rConstants: util::PidConstants) {
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