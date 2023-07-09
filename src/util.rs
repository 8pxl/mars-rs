#[macro_export]
macro_rules! rotate {
    ($x: expr, $y: expr, $t: expr, $p: expr) => {
        {
            let st = $t.sin();
            let ct = $t.cos();
            Vec2{
                x: (ct * $x - st * $y) + $p.0, 
                y: (st * $x + ct * $y) + $p.1
            }
        }
    };
}
#[macro_export]
macro_rules! pub_struct {
    ($name:ident {$($field:ident: $t:ty,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq)] // ewww
        pub struct $name {
            $(pub $field: $t),*
        }
    }
}

pub_struct!(PidConstants {
    p: f32,
    i: f32,
    d: f32,
    tolerance: f32,
    integralThreshold: f32,
    maxIntegral: f32,
});

pub struct Pid {
    prevError: f32,
    derivative: f32,
    integral: f32,
    constants: PidConstants
}

impl Pid {
    pub fn new(con: PidConstants) -> Pid {
        Pid {
            prevError:0.0,
            derivative:0.0,
            integral:0.0,
            constants: con
        }
    }

    pub fn out(&mut self, error: f32) -> f32 {
        if error.abs() < self.constants.tolerance {self.integral= 0.0}
        else if error.abs() < self.constants.integralThreshold {self.integral+= error};
        if self.integral > self.constants.maxIntegral {self.integral= self.constants.maxIntegral};
        self.derivative = error - self.prevError;
        self.prevError = error;
        error * self.constants.p  + self.integral* self.constants.i + self.derivative * self.constants.d
    }
}

impl PidConstants {
    pub fn new() -> PidConstants {
        PidConstants {
            p:0.0,
            i:0.0,
            d:0.0,
            tolerance:0.0,
            integralThreshold:0.0,
            maxIntegral: 0.0
        }
    }
}

pub fn dist(a: (f32, f32), b: (f32, f32)) -> f32{
    (a.0 - b.0).hypot(b.1 - a.1)
}

pub fn absoluteAngleToPoint(p1: (f32, f32), p2: (f32, f32)) -> f32{
 (p2.1 - p1.1).atan2(p2.0 - p1.0)
}

pub fn dirToSpin(target: f32, current: f32) -> i16{
    let d = target - current;
    let diff = if d < 0.0 {d + 360.0} else {d};
    if diff > 180.0 {1} else {-1}
}

pub fn minError(target: f32, current: f32) -> f32 {
    let b = target.max(current);
    let s = target.min(current);
    let diff = b - s;
    
    if diff <= 180.0 {diff} else {((360.0-b) + s) * dirToSpin(target, current) as f32}
}

pub(crate) use rotate;
