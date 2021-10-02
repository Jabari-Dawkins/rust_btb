fn main() {
    // Mutability
    let mut x: u8 = 5;
    println!("The value of x is {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // Shadowing
    let y: u8 = 5;
    println!("The value of y is {}", y);
    let y: i8 = -6;
    println!("The value of y is {}", y);
}
