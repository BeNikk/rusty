fn main() {
    let rect = Shape::Rectangle(2.0, 3.0);
    println!("the area of rectangle is {}", calculate_area(rect));
    let circle = Shape::Circle(10.0);
    println!("the area of circle is {}", calculate_area(circle));
}
enum Shape {
    Rectangle(f64, f64),
    Circle(f64),
}

fn calculate_area(shape: Shape) -> f64 {
    match shape {
        Shape::Rectangle(a, b) => a * b,
        Shape::Circle(a) => 3.14 * a * a,
    }
}
