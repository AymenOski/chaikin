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
}

impl Point {
    pub fn new(x: f64, y: f64, kind: Poly) -> Self {
        Self {
            x: x,
            y: y,
            kind: kind,
        }
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
    pub fn create(p: Point) -> Self {
        Self {
            polygone: vec![p],
            start: 0,
            end: 0,
            len:1,
        }
    }

    pub fn start(&mut self) -> &mut Point {
        &mut self.polygone[self.start]
    }

    pub fn end(&mut self) -> &mut Point {
        &mut self.polygone[self.end]
    }

    pub fn appendPoint(&mut self, p: Point) {
        self.polygone.push(p);
        self.end += 1;
    }

    pub fn append_point(&mut self, p: Point) {
        self.polygone.push(p);
        self.end += 1;
        self.end += 1;
    }

    pub fn append_xy_kind(&mut self, x: f64, y: f64, kind: Poly) {
        self.polygone.push(Point {
            x: x,
            y: y,
            kind: kind,
        });
        self.end += 1;
        self.end += 1;
    }
}
