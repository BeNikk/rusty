mod rectangles;
fn main() {
    let rect = rectangles::Rectangle {
        width: 32,
        height: 32,
    };
    println!("rect is {:#?}", rect);
    println!(
        "Width of rectange is {} and height of rectangle is {}",
        rect.width, rect.height
    );
    let rect2 = rectangles::Rectangle {
        width: 16,
        height: 16,
    };
    println!("The area of the rectangle is {}", rect.area());
    println!(
        "can rect1 contain rect2 -> {} ",
        rect.contains_other(&rect2)
    );
}
