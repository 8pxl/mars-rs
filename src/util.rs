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

pub(crate) use rotate;