use crate::robot;
use crate::util;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};


pub fn pidMTPVel(position: (f32, f32), heading: f32, target: (f32, f32), rotationBias: f32,  lCont: &mut util::Pid, rCont: &mut util::Pid) -> (f32, f32) {
    let linearError = util::dist(position,target);
    let targetHeading = util::absoluteAngleToPoint(position, target).to_degrees();
    let rotationError = util::minError(targetHeading, heading.to_degrees());
    let cre = if rotationError.abs() > 90.0 {0.1} else {rotationError.to_radians().cos()};
    let angularVel = rCont.out(rotationError);
    let linearVel = cre * lCont.out(linearError);
    let rVel = (linearVel - (angularVel.abs() * rotationBias)) + angularVel;
    let lVel = (linearVel - (angularVel.abs() * rotationBias)) - angularVel;
    (lVel, rVel)
}

pub fn pidMTP(robot: Arc<Mutex<robot::Robot>>, target: (f32, f32), rotationBias: f32, timeout: u16, lConstants: util::PidConstants, rConstants: util::PidConstants) {
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);
    while start.elapsed().as_millis() < timeout.into() {
        // self.step(self.pidMTPVel(target, rotationBias, &mut lCont, &mut rCont));
        // self.step((0.1,0.1));
        {
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let heading = robot.heading;
            // robot.step(pidMTPVel(pos, heading, target, rotationBias, &mut lCont, &mut rCont));
            let vel = rCont.out(util::minError(heading.to_degrees(), 90.0));
            println!("{}", heading.to_degrees());
            robot.step((-vel,vel));
        }

        thread::sleep(Duration::from_millis(10));
    }
}