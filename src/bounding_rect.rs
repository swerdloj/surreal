/// Defines the layout-bounds for view elements
pub struct BoundingRect {
    /// Top-left x coord
    pub x: i32,
    /// Top-left y coord
    pub y: i32,
    pub width: u32,
    pub height: u32,
    // pub center: (i32, i32),
}

impl BoundingRect {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            // center: (0, 0),
        }
    }

    // TODO: This + require use of this
    pub fn set_bounds(&mut self, x: i32, y: i32, width: u32, height: u32) {

    }

    // TODO: Store this and calculate automatically. Require user to 
    // adjust BoundingRect via `rect.set_bounds()` which sets center
    pub fn center(&self) -> (i32, i32) {
        (self.x + self.width as i32/2, self.y + self.height as i32/2)
    }

    /// Returns whether a point is within a rect
    pub fn contains(&self, x: i32, y: i32) -> bool {
           x >= self.x 
        && x <= self.x + self.width as i32
        && y >= self.y
        && y <= self.y + self.height as i32
    }

    pub fn top_left(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}