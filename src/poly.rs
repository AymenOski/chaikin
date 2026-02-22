#[derive(Debug, Clone, PartialEq, Eq)]
#[allow(dead_code)]
#[allow(unused)]

pub enum Poly {
    P,
    Q,
    R,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub kind: Poly,
    pub step: u8,
    
}

impl Point {
    pub fn new(x: f64, y: f64, kind: Poly, step: u8) -> Self {
        Self {
            x: x,
            y: y,
            kind: kind,
            step: step,
        }
    }

}

#[derive(Debug, Clone)]
pub struct Polygone {
    pub polygone: Vec<Point>,
    pub start: usize,
    pub end: usize,
    pub len: usize,
}

impl Polygone {
    // Linear interpolation helper: a + (b - a) * t
    fn lerp(a: f64, b: f64, t: f64) -> f64 {
        a + (b - a) * t
    }

    pub fn new(p: Point) -> Self {
        Self {
            polygone: vec![p],
            start: 0,
            end: 0,
            len: 1,
        }
    }

    pub fn empty() -> Self {
        Self {
            polygone: vec![],
            start: 0,
            end: 0,
            len: 0,
        }
    }

    pub fn delete(&mut self) {
        self.polygone.clear();
        self.start = 0;
        self.end = 0;
        self.len = 0;
    }

    pub fn append_point(&mut self, p: Point) {
        self.polygone.push(p);
        self.end += 1;
        self.len += 1;
    }

    pub fn insert_point(&mut self, position: usize, p: Point) {
        self.polygone.insert(position, p);
        self.end += 1;
        self.len += 1;
    }

    pub fn cut_corners(&mut self) {
        if self.len < 3 {
            return;
        }

        for depth in 0..7 {
            let mut i = 0;
            while i < self.polygone.len() - 1 {
                // cut only edges of the current depth aka not the already cut ones
                if self.polygone[i].step != depth {
                    i += 1;
                    continue;
                }
                // Find next point at the SAME depth (skip over old points from previous depths)
                let mut next = i + 1;
                while next < self.polygone.len() && self.polygone[next].step != depth {
                    next += 1;
                }
                
                // No valid next point at this depth, nothing to cut
                if next >= self.polygone.len() {
                    i += 1;
                    continue;
                }
                
                let qx = Self::lerp(self.polygone[i].x, self.polygone[next].x, 0.25);
                let qy = Self::lerp(self.polygone[i].y, self.polygone[next].y, 0.25);
                let q = Point::new(qx, qy, Poly::Q, depth+1);

                let rx = Self::lerp(self.polygone[i].x, self.polygone[next].x, 0.75);
                let ry = Self::lerp(self.polygone[i].y, self.polygone[next].y, 0.75);
                let r = Point::new(rx, ry, Poly::R, depth+1);

                self.insert_point(i + 1, r);
                self.insert_point(i + 1, q);

                i += 1;
            }
        }
    }
}
