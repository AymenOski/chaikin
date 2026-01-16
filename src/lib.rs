use speedy2d::{
    color::Color,
    dimen::Vector2,
    window::{MouseButton, VirtualKeyCode, WindowHandler, WindowHelper},
    Graphics2D,
};
use std::time::{Duration, Instant};

pub const WIDTH: f32 = 860.0;
pub const HEIGHT: f32 = 800.0;
const MAX_STEPS: usize = 7;
const CLICK_RADIUS: f32 = 10.0;
const POINT_OUTER_R: f32 = 5.0;
const POINT_INNER_R: f32 = 3.5;
const ANIM_INTERVAL: Duration = Duration::from_millis(1100);

#[derive(Clone, Copy, Debug)]
struct Pt {
    x: f32,
    y: f32,
}

impl From<Vector2<f32>> for Pt {
    fn from(v: Vector2<f32>) -> Self {
        Self { x: v.x, y: v.y }
    }
}

impl From<Pt> for Vector2<f32> {
    fn from(p: Pt) -> Self {
        Vector2::new(p.x, p.y)
    }
}

fn dist2(a: Pt, b: Pt) -> f32 {
    let (dx, dy) = (a.x - b.x, a.y - b.y);
    dx * dx + dy * dy
}

fn chaikin_step(points: &[Pt], closed: bool) -> Vec<Pt> {
    let n = points.len();
    if n < 2 {
        return points.to_vec();
    }

    let mut out = Vec::with_capacity(n * 2 + 2);

    if closed {
        for i in 0..n {
            let (p0, p1) = (points[i], points[(i + 1) % n]);
            out.push(Pt {
                x: p0.x * 0.75 + p1.x * 0.25,
                y: p0.y * 0.75 + p1.y * 0.25,
            });
            out.push(Pt {
                x: p0.x * 0.25 + p1.x * 0.75,
                y: p0.y * 0.25 + p1.y * 0.75,
            });
        }
    } else {
        out.push(points[0]);
        for w in points.windows(2) {
            let (p0, p1) = (w[0], w[1]);
            out.push(Pt {
                x: p0.x * 0.75 + p1.x * 0.25,
                y: p0.y * 0.75 + p1.y * 0.25,
            });
            out.push(Pt {
                x: p0.x * 0.25 + p1.x * 0.75,
                y: p0.y * 0.25 + p1.y * 0.75,
            });
        }
        out.push(points[n - 1]);
    }

    out
}
