mod options;
mod result;
fn main() {
    let rect = Shape::Rectangle(2.0, 3.0);
    println!("the area of rectangle is {}", calculate_area(rect));
    let circle = Shape::Circle(10.0);
    println!("the area of circle is {}", calculate_area(circle));
    let name = String::from("Alaska");
    let index = options::find_first(name);
    match index {
        Some(a) => println!("{}", a),
        None => println!("There is no A in the name"),
    }
    result::read_file("a.txt");
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
