#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}
impl Rectangle {
    pub fn area(&self) -> u32 {
        self.width * self.height
    }
    pub fn contains_other(&self, other: &Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            return true;
        }
        false
    }
}
