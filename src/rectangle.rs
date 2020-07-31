pub struct Rectangle {
    /// Top-left x coord
    x: i32,
    /// Top-left y coord
    y: i32,
    width: u32,
    height: u32,
}

impl Rectangle {
    pub fn new() -> Self {
        Rectangle {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }
}