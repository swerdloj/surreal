/// Defines the layout-bounds for view elements
pub struct BoundingRect {
    /// Top-left x coord
    pub x: i32,
    /// Top-left y coord
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl BoundingRect {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn top_left(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}