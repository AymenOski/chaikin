mod poly;

use poly::{Point, Poly, Polygone};

// write the main code in the func run() to use the lib crate instead of the binary crate idomatic rust XD
pub fn run() {
    let p1 = Point::new(1, 1, Poly::P);
    let p2 = Point::new(2, 2, Poly::P);
    println!("{}", p1.get_distance(p2.clone()));
    // let polygone = Polygone::create(p.clone());
    // println!("{:#?}", polygone);

    println!("main thread should be written here!");
}
