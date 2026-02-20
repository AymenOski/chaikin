mod poly;

use poly::{Point, Poly, Polygone};

// write the main code in the func run() to use the lib crate instead of the binary crate idomatic rust XD
pub fn run() {
    let p = Point::new(1, 1, Poly::P);
    println!("{:#?}", p);
    let polygone = Polygone::create(p.clone());
    println!("{:#?}", polygone);

    println!("main thread should be written here!");
}
