#[derive(Debug, Clone)]
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

    pub fn get_distance(&self, p: Point) -> f64 {
        let dx = (p.x - self.x).powf(2.0);
        let dy = (p.y - self.y).powf(2.0);
        dx.hypot(dy)
    }
}

#[derive(Debug)]
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

    pub fn start(&mut self) -> &mut Point {
        &mut self.polygone[self.start]
    }

    pub fn end(&mut self) -> &mut Point {
        &mut self.polygone[self.end]
    }

    pub fn append_point(&mut self, p: Point) {
        self.polygone.push(p);
        self.end += 1;
        self.len += 1;
    }

    pub fn append_xy_kind(&mut self, x: f64, y: f64, kind: Poly, step: u8) {
        self.polygone.push(Point {
            x: x,
            y: y,
            kind: kind,
            step: step,
        });
        self.end += 1;
        self.len += 1;
    }

    pub fn insert_point(&mut self, position: usize, p: Point) {
        self.polygone.insert(position, p);
        self.end += 1;
        self.len += 1;
    }

    pub fn insert_xy_kind(&mut self, position: usize, x: f64, y: f64, kind: Poly, step: u8) {
        self.polygone.insert(
            position,
            Point {
                x: x,
                y: y,
                kind: kind,
                step: step,
            },
        );
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
                // Find all points created at THIS depth level
                if self.polygone[i].step != depth {
                    i += 1;
                    continue;
                }
                // Create Q and R by cutting edges starting from these points
                let qx = Self::lerp(self.polygone[i].x, self.polygone[i + 1].x, 0.25);
                let qy = Self::lerp(self.polygone[i].y, self.polygone[i + 1].y, 0.25);
                let q = Point::new(qx, qy, Poly::Q, depth+1);

                let rx = Self::lerp(self.polygone[i].x, self.polygone[i + 1].x, 0.75);
                let ry = Self::lerp(self.polygone[i].y, self.polygone[i + 1].y, 0.75);
                let r = Point::new(rx, ry, Poly::R, depth+1);

                self.insert_point(i + 1, r);
                self.insert_point(i + 1, q);

                i += 1;
            }
        }
    }
}
