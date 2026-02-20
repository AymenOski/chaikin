#[derive(Debug, Clone)]
// after this we are going to allow dead code and any unused warnings or constructs
#[allow(dead_code)]
#[allow(unused)]

pub enum Poly {
    P,
    Q,
    R,
}

#[derive(Debug, Clone)]
pub struct Point {
    pub x: i32,
    pub y: i32,
    pub kind: Poly,
}

impl Point {
    pub fn new(x: i32, y: i32, kind: Poly) -> Self {
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

    pub fn append_xy_kind(&mut self, x: i32, y: i32, kind: Poly) {
        self.polygone.push(Point {
            x: x,
            y: y,
            kind: kind,
        });
        self.end += 1;
        self.end += 1;
    }
}
