use std::f64::consts::{PI};
#[derive(Debug)]
struct Rectangle {
    width: f64,
    height: f64,
}
impl Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

#[derive(Debug)]
struct Circle {
    radius: f64,
}
impl Circle {
    fn area(&self) -> f64 {
        PI * self.radius.powi(2)
    }
}

#[derive(Debug)]
struct Triangle {
    base: f64,
    height: f64,
}
impl Triangle {
    fn area(&self) -> f64 {
        (self.base * self.height) / 2.0
    }
}


fn main() {
    let rect1 = Rectangle {
        width: 30.0,
        height: 50.0
    };

    let circ1 = Circle {
        radius: 10.0
    };

    let trng1 = Triangle {
        base: 5.0,
        height: 4.0
    };

    println!(
        "The area of the rectangle is {} square units.",
        rect1.area()
    );

    println!(
        "The area of the circle is {} square units.",
        circ1.area()
    );

    println!(
        "The area of the triangle is {} square units.",
        trng1.area()
    )
}
