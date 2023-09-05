const MAX_VEL: f32 = 4.0;
const MAX_AVEL: f32 = 0.07;
use macroquad::prelude::*;

// pub fn drive(vel: &mut(f32, f32), robot: &mut super::robot::Robot) {
//     let absv = vel.0.abs();
//     let absv1 = vel.1.abs();
//     let sgnv = vel.0.signum();
//     let sgnv1 = vel.1.signum();

//     if absv > MAX_VEL {
//         vel.0 = MAX_VEL * sgnv
//     }
//     if absv1 > MAX_VEL {
//         vel.1 = MAX_VEL * sgnv1
//     }

//     vel.0 = if absv > 0.07 {vel.0 - sgnv * 0.07} else {0.0};
//     vel.1 = if absv1 > 0.07 {vel.1 - sgnv1 * 0.07} else {0.0};

//     if is_key_down(KeyCode::W) { 
//         vel.0 += 0.2;
//         vel.1 += 0.2;
//     }
//     if is_key_down(KeyCode::A) { 
//         vel.1 += 0.2 * -sgnv1;
//         vel.0 -= 0.2 * -sgnv1;
//     }
//     if is_key_down(KeyCode::S) { 
//         vel.0 -= 0.2;
//         vel.1 -= 0.2;
//     }
//     if is_key_down(KeyCode::D) { 
//         vel.0 += 0.2 * -sgnv;
//         vel.1 -= 0.2 * -sgnv;
//     }

//     robot.step(*vel);
// }

// (l+r) / 2 = v
// (l-r) / t = w
pub fn drive(vel: &mut(f32, f32), robot: &mut super::robot::Robot) {
    let track = robot.robotSize;
    
    let absv = vel.0.abs();
    let absv1 = vel.1.abs();
    let sgnv = vel.0.signum();
    let sgnv1 = vel.1.signum();

    if absv > MAX_VEL {
        vel.0 = MAX_VEL * sgnv
    }
    if absv1 > MAX_AVEL {
        vel.1 = MAX_AVEL * sgnv1
    }

    vel.0 = if absv > 0.07 {vel.0 - sgnv * 0.07} else {0.0};
    vel.1 = if absv1 > 0.005 {vel.1 - sgnv1 * 0.005} else {0.0};

    if is_key_down(KeyCode::W) { 
        vel.0 += 0.2;
    }
    if is_key_down(KeyCode::A) { 
        vel.1 += 0.0076;
    }
    if is_key_down(KeyCode::S) { 
        vel.0 -= 0.2;
    }
    if is_key_down(KeyCode::D) { 
        vel.1 -= 0.0076;
    }
    let vr = (2.0 * vel.0 - track * vel.1) / 2.0;
    let vl = vr + (track * vel.1);

    robot.step((vl,vr));
}