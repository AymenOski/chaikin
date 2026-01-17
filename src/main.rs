use chaikin::*;
fn main() {
    let polygon = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 4.0, y: 0.0 },
        Point { x: 4.0, y: 4.0 },
        Point { x: 0.0, y: 4.0 },
    ];

    println!("Original points:");
    for p in &polygon {
        println!("{:?}", p);
    }

    let smooth = chaikin_algo(&polygon);

    println!("\nAfter Chaikin algorithm:");
    for p in &smooth {
        println!("{:?}", p);
    }
}
