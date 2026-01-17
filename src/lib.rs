#[derive(Debug, Clone, Copy)]

pub struct Point {
    pub x: f64,
    pub y: f64,
}
pub fn chaikin_algo(points: &[Point]) -> Vec<Point> {
    let mut res = Vec::new();
    if points.len() < 3 {
        return res;
    }
    for i in 0..points.len() - 1 {
        let p0 = &points[i];
        let p1 = &points[i + 1];
        let q = Point {
            x: 0.75 * p0.x + 0.25 * p1.x,
            y: 0.75 * p0.y + 0.25 * p1.y,
        };
        let r = Point {
            x: 0.25 * p0.x + 0.75 * p1.x,
            y: 0.25 * p0.y + 0.75 * p1.y,
        };
        res.push(q);
        res.push(r);
    }
    res
}
