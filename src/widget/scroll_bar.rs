use crate::state::State;
use crate::bounding_rect::BoundingRect;

pub struct ScrollBar<Msg> {
    id: &'static str,
    container_bounds: BoundingRect,
    slider_bounds: BoundingRect,
    orientation: crate::Orientation,
    // size of the slidable region
    slider_size: u32,
    // width of the bar (in the unscrollable direction)
    slider_width: u32,
    // fn(scroll_percentage, state)..
    on_scroll: Option<Box<dyn FnMut(f32, std::cell::RefMut<State>) -> Msg>>,
    // Scroller
    slider_roundness: Option<f32>,
    // Scroll container
    container_roundness: Option<f32>,
    // Whether the slider has a container
    // TODO: Make this `background_transparency` and default to 0.0
    has_container: bool,

    container_color: Option<crate::Color>,
    slider_color: Option<crate::Color>,

    is_held_down: bool,

    should_resize: bool,

    last_percentage: f32,
}

impl<Msg: crate::EmptyMessage> ScrollBar<Msg> {
    /// - `container_size`: the size of the scroll bar
    /// - `slider_size`: the size of the slider within the scroll bar
    pub fn new(id: &'static str, container_size: u32, slider_size: u32) -> Self {
        let mut container_bounds = BoundingRect::new();
        container_bounds.width = container_size;
        container_bounds.height = container_size;

        let mut slider_bounds = BoundingRect::new();
        slider_bounds.width = slider_size;
        slider_bounds.height = slider_size;
        
        Self {
            id,
            container_bounds,
            slider_bounds,
            slider_size,
            orientation: crate::Orientation::Vertical,
            on_scroll: None,
            slider_roundness: None,
            slider_width: 12,
            container_roundness: None,
            has_container: true,

            container_color: None,
            slider_color: None,

            is_held_down: false,
            should_resize: false,

            last_percentage: 0.0,
        }
    }

    pub fn width(mut self, width: u32) -> Self {
        self.slider_width = width;
        self
    }

    pub fn orientation(mut self, orientation: crate::Orientation) -> Self {
        self.orientation = orientation;
        self
    }

    pub fn container_color(mut self, color: crate::Color) -> Self {
        self.container_color = Some(color);
        self
    }

    pub fn on_scroll<F: FnMut(f32, std::cell::RefMut<State>) -> Msg + 'static>(mut self, on_scroll: F) -> Self {
        self.on_scroll = Some(Box::new(on_scroll));
        self
    }

    /// slider % -> position
    fn set_slider_position_from_last_percentage(&mut self) {
        match self.orientation {
            crate::Orientation::Vertical => {
                self.slider_bounds.y = (self.last_percentage * (self.container_bounds.height - self.slider_bounds.height) as f32).round() as i32 + self.container_bounds.y;
            }
            crate::Orientation::Horizontal => {
                self.slider_bounds.x = (self.last_percentage * (self.container_bounds.width - self.slider_bounds.width) as f32).round() as i32 + self.container_bounds.x;
            }
        }
    }

    /// Called when bar is scrolled
    fn scroll(&mut self, change: i32, state: std::cell::RefMut<State>, message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        // FIXME: Swap the `if` and `else if` branches for better performance
        let percent = match self.orientation {
            // left/right -> use x
            crate::Orientation::Horizontal => {
                self.slider_bounds.x += change;

                // Clamp to boundary
                if self.slider_bounds.width as i32 + self.slider_bounds.x > self.container_bounds.width as i32 + self.container_bounds.x {
                    self.slider_bounds.x = self.container_bounds.x + self.container_bounds.width as i32 - self.slider_bounds.width as i32;
                } else if self.slider_bounds.x < self.container_bounds.x {
                    self.slider_bounds.x = self.container_bounds.x;
                }

                (self.slider_bounds.x - self.container_bounds.x) as f32 / (self.container_bounds.width as i32 - self.slider_bounds.width as i32) as f32
            }
            // up/down -> use y
            crate::Orientation::Vertical => {
                self.slider_bounds.y += change;

                // Clamp to boundary
                if self.slider_bounds.height as i32 + self.slider_bounds.y > self.container_bounds.height as i32 + self.container_bounds.y {
                    self.slider_bounds.y = self.container_bounds.y + self.container_bounds.height as i32 - self.slider_bounds.height as i32;
                } else if self.slider_bounds.y < self.container_bounds.y {
                    self.slider_bounds.y = self.container_bounds.y;
                }
                
                (self.slider_bounds.y - self.container_bounds.y) as f32 / (self.container_bounds.height as i32 - self.slider_bounds.height as i32) as f32
            }
        };

        // If the scroll bar moved
        let scrolled = self.last_percentage != percent;
        self.last_percentage = percent;

        if scrolled {
            if let Some(on_scroll) = &mut self.on_scroll {
                message_queue.push( (on_scroll)(percent, state) );
            }
            crate::EventResponse::Redraw
        } else {
            crate::EventResponse::Consume
        }
    }
}

impl<Msg: crate::EmptyMessage> super::Widget<Msg> for ScrollBar<Msg> {
    fn id(&self) -> &'static str {
        self.id
    }

    // TODO: Listen for scroll wheel
    fn handle_event(&mut self, event: &crate::event::ApplicationEvent, state: std::cell::RefMut<State>, message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        use crate::event::*;
        
        match event {
            ApplicationEvent::MouseMotion { relative_change: (dx, dy), .. } => {
                if self.is_held_down {
                    let change = if self.orientation.is_horizontal() { dx } else { dy };
                    return self.scroll(*change, state, message_queue);
                }

                crate::EventResponse::None
            }

            ApplicationEvent::MouseButton { state: ButtonState::Pressed, button: MouseButton::Left, position: (x, y) } => {
                // TODO: Differentiate slider & container bounds.
                // Clicking on container should scroll to that point automatically.
                if self.slider_bounds.contains(*x, *y) {
                    self.is_held_down = true;
                    return crate::EventResponse::Consume;
                } else if self.container_bounds.contains(*x, *y) {
                    // TODO: Scroll to the point
                }

                crate::EventResponse::None
            }

            ApplicationEvent::MouseButton { state: ButtonState::Released, button: MouseButton::Left, position: (x, y) } => {
                self.is_held_down = false;

                crate::EventResponse::None
            }

            _ => {
                crate::EventResponse::None
            }
        }
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
    }

    fn init(&mut self, _renderer: &mut crate::render::Renderer, theme: &crate::style::Theme) {
        // Size the slider
        match self.orientation {
            crate::Orientation::Vertical => {
                self.slider_bounds.width = self.slider_width;
                self.container_bounds.width = self.slider_width;
            }
            
            crate::Orientation::Horizontal => {
                self.slider_bounds.height = self.slider_width;
                self.container_bounds.height = self.slider_width;
            }
        }
        
        if self.container_color.is_none() {
            self.container_color = Some(theme.colors.secondary);
        }

        if self.slider_color.is_none() {
            self.slider_color = Some(theme.colors.primary);
        }

        if self.slider_roundness.is_none() {
            self.slider_roundness = Some(theme.widget_styles.buttons.roundness);
        }

        if self.container_roundness.is_none() {
            self.container_roundness = Some(theme.widget_styles.buttons.roundness);
        }
    }

    fn place(&mut self, x: i32, y: i32) {
        self.container_bounds.x = x;
        self.container_bounds.y = y;

        self.slider_bounds.x = x;
        self.slider_bounds.y = y;
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.container_bounds.x += dx;
        self.container_bounds.y += dy;
        
        self.slider_bounds.x += dx;
        self.slider_bounds.y += dy;

        self.set_slider_position_from_last_percentage();
    }

    fn render_size(&self, _theme: &crate::style::Theme) -> (u32, u32) {
        // countainer would always be larger than the slider
        if self.has_container {
            self.container_bounds.dimensions()
        } else {
            self.slider_bounds.dimensions()
        }
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        // Container
        if self.has_container {
            renderer.draw(crate::render::DrawCommand::RoundedRect {
                top_left: self.container_bounds.top_left(),
                width: self.container_bounds.width,
                height: self.container_bounds.height,
                roundness_percent: self.container_roundness.unwrap(),
                color: self.container_color.unwrap(),
            });
        }

        // Slider
        renderer.draw(crate::render::DrawCommand::RoundedRect {
            top_left: self.slider_bounds.top_left(),
            width: self.slider_bounds.width,
            height: self.slider_bounds.height,
            roundness_percent: self.slider_roundness.unwrap(),
            color: self.slider_color.unwrap(),
        });
    }
}