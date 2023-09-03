use macroquad::prelude::mouse_position;
use macroquad::shapes::draw_circle;
use macroquad::shapes::draw_line;

use crate::robot;
use crate::util;
use crate::util::absoluteAngleToPoint;
use crate::paths;
use std::f32::consts::PI;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

//https://www.desmos.com/calculator/vhte05bsot for scaling
pub fn pidMTPVel(position: (f32, f32), heading: f32, target: (f32, f32), rotationCut: f32,  lCont: &mut util::Pid, rCont: &mut util::Pid, min: f32) -> (f32, f32) {
    let linearError = util::dist(position,target);
    let targetHeading = util::absoluteAngleToPoint(position, target);
    let rotationError = util::minError(targetHeading, if heading < 0.0 {360.0 + heading} else {heading});
    let scale = 90.0 / rotationCut;
    let cre = if rotationError.abs() > rotationCut {0.1} else {(scale * rotationError.to_radians()).cos()};
    let angularVel = rCont.out(rotationError);
    let linearVel = (cre * lCont.out(linearError)).max(min);
    let rVel = linearVel - angularVel;
    let lVel = linearVel + angularVel;
    (lVel, rVel)
}

pub fn pidMTP(robot: &Arc<Mutex<robot::Robot>>, target: (f32, f32), rotationCut: f32, timeout: u16, lConstants: util::PidConstants, rConstants: util::PidConstants, min: f32) {
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);
    while start.elapsed().as_millis() < timeout.into() {
        {
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let heading = robot.heading.to_degrees() % 360.0;
            robot.step(pidMTPVel(pos, heading, target, rotationCut, &mut lCont, &mut rCont, min));
        }

        thread::sleep(Duration::from_millis(10));
    }
}

pub fn boomerang(robot: &Arc<Mutex<robot::Robot>>, target: (f32, f32), timeout: u16, dLead: f32, thetaEnd: f32, rotationCut: f32, lConstants: util::PidConstants, rConstants: util::PidConstants, min: f32)
{
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);

    while start.elapsed().as_millis() < timeout.into() {
        {
            // let target = mouse_position();
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let heading = robot.heading.to_degrees() % 360.0;
            let h = (pos.0 - target.0).hypot(pos.1 - target.1);
            let carrot = (target.0 - (h * thetaEnd.sin() * dLead), target.1 - (h * thetaEnd.cos() * dLead));
            // use macroquad::prelude::RED;
            // use macroquad::prelude::BLACK;
            // draw_line(pos.0,pos.1,carrot.0, carrot.1, 2.0, RED);
            // draw_circle(target.0, target.1, 1.0, BLACK);
            robot.step(pidMTPVel(pos, heading, carrot, rotationCut, &mut lCont, &mut rCont, min));
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn followPath(robot: &Arc<Mutex<robot::Robot>>, path: Vec<(f32,f32)>, timeout: u32, dLead: f32, thetaEnd: f32, rotationCut: f32, lConstants: util::PidConstants, rConstants: util::PidConstants, min: f32) {
    let start = Instant::now();
    let mut lCont = util::Pid::new(lConstants);
    let mut rCont = util::Pid::new(rConstants);
    let mut pathIndex = 0;

    while start.elapsed().as_millis() < timeout.into() {
        {
            let target = path[pathIndex];
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let thetaEnd = absoluteAngleToPoint(path[if (pathIndex as i8 -1) < 0 {0} else {pathIndex-1}], target).to_radians();
            let heading = robot.heading.to_degrees() % 360.0;
            let h = (pos.0 - target.0).hypot(pos.1 - target.1);
            // println!("{}", h);
            let carrot = (target.0 - (h * thetaEnd.sin() * dLead), target.1 - (h * thetaEnd.cos() * dLead));
            // use macroquad::prelude::RED;
            // use macroquad::prelude::BLACK;
            // draw_line(pos.0,pos.1,carrot.0, carrot.1, 2.0, RED);
            // draw_circle(target.0, target.1, 1.0, BLACK);
            robot.step(pidMTPVel(pos, heading, carrot, rotationCut, &mut lCont, &mut rCont, min));
            if util::dist(pos, target) < 40.0 {
                if pathIndex < (path.len()-1) {pathIndex += 1};
            }
        }
        thread::sleep(Duration::from_millis(10));
    }
}

pub fn eulerTurn(robot: &Arc<Mutex<robot::Robot>>, theta: f32, rate: f32, curvature: f32, timeout: u32, dir: i8, constants: util::PidConstants) -> f32{
    let start = Instant::now();
    let mut curvature = curvature;
    let mut controller = util::Pid::new(constants);

    while start.elapsed().as_millis() < timeout.into() {
        {
            curvature += rate;
            let mut robot = robot.lock().unwrap();
            let curr = robot.heading.to_degrees();
            let error = util::minError(theta, curr);
            let sl = error * (1.0 / curvature + 15.0);
            let sr = error * (1.0 / curvature - 15.0);
            let ratio = sl/sr;
            let mut vel = controller.out(error);
            println!("{}", curr);
            vel = if vel.abs() >= 127.0 {127.0 * vel.signum()} else {vel};
            let rvel = (2.0 * vel) / (ratio + 1.0);
            let lvel = ratio * rvel;
            robot.step(if dir.signum() < 0 {(lvel, rvel)} else {(-rvel, -lvel)});
        }
        thread::sleep(Duration::from_millis(10));
    }
    return curvature;
}

enum TargetPoint {
    Last((f32, f32)),
    Other((f32, f32))
}

fn targetPoint(path: &Vec<(f32,f32)>, position: (f32, f32), lookAhead: f32, lineLookAhead: usize, lineIndex: usize) -> TargetPoint {
    //lookAhead: radius of look ahead circle
    //lineLookAhead: how many lines ahead of the path the robot should search
    //lineIndex: the current line the robot is travelling on

    let mut furthestPoint: (f32, f32);
    let mut targetPoint = TargetPoint::Other((0.0, 0.0));
    let mut closestDist = f32::INFINITY;

    //lastLine: furthest line look ahead point
    let lastLine = (lineIndex + lineLookAhead).min(path.len() - 1);
    for i in lineIndex..lastLine {
        furthestPoint = path[lastLine - 1];

        let x1: f32 = path[i].0;
        let y1: f32 = path[i].1;
        let x2: f32 = path[i + 1].0;
        let y2: f32 = path[i+1].1;

        let ox1 = x1 - position.0;
        let oy1 = y1 - position.1;
        let ox2 = x2 - position.0;
        let oy2 = y2 - position.1;

        let dx = ox2 - ox1;
        let dy = oy2 - oy1;
        let dr = dx.hypot(dy);
        let D = ox1*oy2 - ox2 * oy1; 
        let discriminant = lookAhead.powi(2) * dr.powi(2) - D.powi(2);

        if discriminant >= 0.0 {
              let sDiscriminant = discriminant.sqrt();
              let dxdy = D * dy;
              let dxdx = D*dx;
              let sdyxdxxsd = dy.signum() * dx * sDiscriminant;
              let dr2 = dr.powi(2);
              let adyxsd = dy.abs() * sDiscriminant;

              let minX = x1.min(x2);
              let maxX = x1.max(x2);
              let minY = y1.min(y2);
              let maxY = y1.max(y2);

              let sx1 = (dxdy + sdyxdxxsd) / dr2;
              let sy1 = (-dxdx + adyxsd) / dr2;
              let sx2 = (dxdy - sdyxdxxsd) / dr2;
              let sy2 = (-dxdx - adyxsd) / dr2;

              let s1 = (sx1 + position.0, sy1 + position.1);
              let s2 = (sx2 + position.0, sy2 + position.1);

              let s1Valid = s1.0 >= minX && s1.0 <= maxX && s1.1 >= minY && s1.1 <= maxY;
              let s2Valid = s2.0 >= minX && s2.0 <= maxX && s2.1 >= minY && s2.1 <= maxY;

              //if the line index is the last line in the look aheaed, increase the line index
              if (i == (lastLine - 1)) && (lastLine - 1 != path.len()) {
                if s1Valid {
                    return TargetPoint::Last(s1);
                }

                if s2Valid {
                    return TargetPoint::Last(s2);
                }
              }

              let s1Dist = util::dist(s1, furthestPoint);
              let s2Dist = util::dist(s2, furthestPoint);

              if s1Valid && s1Dist < closestDist {
                targetPoint = TargetPoint::Other(s1);
                closestDist = s1Dist;
              }

              if s2Valid && s2Dist < closestDist {
                targetPoint  = TargetPoint::Other(s2);
                closestDist = s2Dist;
              }
        }
    }
    targetPoint
}

pub fn moveToPurePursuit(robot: &Arc<Mutex<robot::Robot>>, path: Vec<(f32,f32)>, lookAhead: f32, lineLookAhead: usize, finalTimeout: u16) {
    let mut lineIndex = 0;
    let mut start = Instant::now();
    let mut lCont = util::Pid::new(util::PidConstants {
        p: 0.05,
        i: 0.0,
        d: 0.0,
        tolerance: 0.0,
        integralThreshold: 0.0,
        maxIntegral: 0.0
    });

    let mut rCont = util::Pid::new(util::PidConstants {
        p: 0.015,
        i: 0.0,
        d: 0.0,
        tolerance: 0.0,
        integralThreshold: 0.0,
        maxIntegral: 0.0
    });

    while start.elapsed().as_millis() < finalTimeout.into() {
        {
            let mut robot = robot.lock().unwrap();
            let pos = robot.position;
            let last = path[path.len() - 1];
            let target;

            if lineIndex == path.len() - 2 {
                target = last;
            }
            
            else {
                start = Instant::now();

                let targ = targetPoint(&path, pos, lookAhead, lineLookAhead, lineIndex);
                
                match targ {
                    TargetPoint::Last(point) => {
                        lineIndex += 1;
                        target = point;
                    }
                    
                    TargetPoint::Other(point) => {
                        target = point;
                    }
                }
            }

            let heading = robot.heading.to_degrees() % 360.0;
            robot.step(pidMTPVel(pos, heading, target, 90.0, &mut lCont, &mut rCont, 0.0));
            // use macroquad::prelude::BLACK;
            // draw_line(pos.0, pos.1, target.0, target.1, 2.0, BLACK);
        }

        thread::sleep(Duration::from_millis(10));
    }
    // moveTo(path[-1],finalTimeout)
}

pub fn bezier2dMotionProfile(path: paths::Bezier, maxSpeed: f32, accel: f32, decel: f32, resolution: i32, track: f32) {
    let len = path.length();
    let max = ((2.0 * accel * decel * len) / (accel + decel)).sqrt().min(maxSpeed);
    let accelTime = max / accel;
    let decelTime = max / decel;
    let accelDist = (max / 2.0) * accelTime;
    let decelDist = (max / 2.0) * decelTime;

    let mut profile: Vec<f32> = Vec::new();
    let mut dist = 0.0;
    let mut t = 0.0;
    while dist < len {
        
        let curvature = path.curvature(t);
        if dist > len {
            dist = len;
        }
        let mut vel = 
            if dist < accelDist {
                (2.0 * accel * dist).sqrt()
            }

            else if dist < (len - decelDist) {
                max
            }
            
            else {
                max.powi(2) + 2.0 * decel * dist
            };
        let left = (vel * (2.0 + (curvature * track))) / 2.0;
        let right = (vel * (2.0 - (curvature * track))) / 2.0;
        if left.max(right) > (2.0 * maxSpeed - left.min(right)) {

        }
    }
}


// double max = std::min(std::sqrt((2 * accel * decel * dist) / accel + decel), maxSpeed);
// double accelTime = max / accel;
// double decelTime = max / decel;
// double coastDist = (dist / max) - (max / (2 * accel)) - (max / (2*decel));
// double coastTime = coastDist / max;
// double totalTime = accelTime + decelTime + coastTime;