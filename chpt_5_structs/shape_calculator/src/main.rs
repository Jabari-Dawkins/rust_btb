use std::f64::consts::{PI};
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}
impl Circle {
    fn area(&self) -> f64 {
        2.0 * PI * self.radius
    }
}



fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50
    };

    let circ1 = Circle {
        radius: 10.0
    };

    println!(
        "The area of the rectangle is {} square units.",
        rect1.area()
    );

    println!(
        "The area of the circle is {} square units.",
        circ1.area()
    );
}
