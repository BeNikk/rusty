fn main() {
    let cat = Cat {
        name: String::from("Cat"),
    };
    cat.speak();
    let rect = Rect {
        width: 32.0,
        height: 2.0,
    };
    println!("area is {}", rect.area());
    let circ = Circle { radius: 32.0 };
    println!("the area of circle is {}", circ.area());
}

trait Speak {
    fn speak(&self);
}
struct Dog {
    name: String,
}
struct Cat {
    name: String,
}

impl Speak for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}

trait area {
    fn area(&self) -> f64 {
        1.0
    }
}
struct Rect {
    width: f64,
    height: f64,
}
struct Circle {
    radius: f64,
}

impl area for Rect {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}
impl area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
