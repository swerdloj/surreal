use crate::bounding_rect::BoundingRect;
use crate::state::State;
use crate::view_element::*;

use std::cell::RefMut;

use super::{Widget, Text, Image};

pub enum Contents<Msg> {
    Char(Text<Msg>),
    // TODO: This (once image support is added)
    Image(Image<Msg>),
    None,
}

pub struct CircleButton<Msg> {
    id: &'static str,
    bounds: BoundingRect,
    contents: Contents<Msg>,
    on_click: Option<Box<dyn FnMut(RefMut<State>) -> Msg>>,
    radius: u32,
    color: Option<crate::Color>,
    mouse_down_in_bounds: bool,
    should_resize: bool,
}

impl<Msg> CircleButton<Msg> {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            bounds: BoundingRect::new(),
            contents: Contents::None,
            on_click: None,
            // Zero implies uninitialized
            radius: 0,
            color: None,
            mouse_down_in_bounds: false,
            should_resize: false,
        }
    }

    pub fn on_click<F: FnMut(RefMut<State>) -> Msg + 'static>(mut self, on_click: F) -> Self {
        self.on_click = Some(Box::new(on_click));
        self
    }

    pub fn color(mut self, color: crate::Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn radius(mut self, radius: u32) -> Self {
        self.radius = radius;
        self
    }

    pub fn character(mut self, character: crate::widget::text::TextCharacter<Msg>) -> Self {
        if let Contents::Image(_) = self.contents {
            println!("WARNING: Overwriting image resource of `{}` with a character", self.id);
        }

        self.contents = Contents::Char(character.into());
        self
    }

    pub fn image(mut self, image: Image<Msg>) -> Self {
        if let Contents::Char(_) = self.contents {
            println!("WARNING: Overwriting character resource of `{}` with an image", self.id);
        }

        self.contents = Contents::Image(image);
        self
    }

    fn contains_point(&self, x: i32, y: i32) -> bool {
        let center = self.bounds.center();

        (((x - center.0).abs().pow(2) + (y - center.1).abs().pow(2)) as f32).sqrt() < self.radius as f32
    }
}

impl<Msg: EmptyMessage> Widget<Msg> for CircleButton<Msg> where Msg: 'static {
    fn id(&self) -> &'static str {
        self.id
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
    }

    fn place(&mut self, x: i32, y: i32) {
        self.bounds.x = x;
        self.bounds.y = y;

        match &mut self.contents {
            Contents::Char(text) => {
                let (text_width, text_height) = text.bounds.dimensions();
                text.place(x, y);

                // FIXME: `place` doesn't seem like the proper location for this
                text.translate(
                (self.bounds.width / 2) as i32 - (text_width / 2) as i32, 
                (self.bounds.height / 2) as i32 - (text_height / 2) as i32
                );
            }

            Contents::Image(image) => {
                let (image_width, image_height) = image.bounds.dimensions();
                image.place(x, y);

                // FIXME: `place` doesn't seem like the proper location for this
                image.translate(
                (self.bounds.width / 2) as i32 - (image_width / 2) as i32, 
                (self.bounds.height / 2) as i32 - (image_height / 2) as i32
                );
            }

            Contents::None => {}
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.bounds.x += dx;
        self.bounds.y += dy;

        match &mut self.contents {
            Contents::Char(text) => {
                text.translate(dx, dy);
            }
            Contents::Image(image) => {
                image.translate(dx, dy);
            }
            Contents::None => {}
        }
    }

    fn init(&mut self, renderer: &mut crate::render::Renderer, theme: &crate::style::Theme) {
        match &mut self.contents {
            Contents::Char(text) => {
                text.init(renderer, theme);
            }

            Contents::Image(image) => {
                // Scale the image to fit within the button with the given padding
                let (width, height) = renderer.texture_map.get_resource_dimensions(&image.resource);
                if width > height {
                    image.set_scaled_width(self.radius * 2 - 2 * theme.widget_styles.internal_padding.horizontal);
                } else {
                    image.set_scaled_height(self.radius * 2 - 2 * theme.widget_styles.internal_padding.vertical);
                }
                image.init(renderer, theme);
            }
            
            Contents::None => {}
        }

        if self.color.is_none() {
            self.color = Some(theme.colors.primary);
        }

        if self.radius == 0 {
            self.radius = theme.widget_styles.buttons.circle_button_radius;
        }


        self.bounds.width = self.radius * 2;
        self.bounds.height = self.radius * 2;
    }

    fn handle_event(&mut self, event: &crate::event::ApplicationEvent, state: std::cell::RefMut<crate::state::State>, message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        use crate::event::*;
        
        match event {
            ApplicationEvent::MouseButton { state: ButtonState::Pressed, button: MouseButton::Left, position: (x, y) } => {
                if self.contains_point(*x, *y) {
                    self.mouse_down_in_bounds = true;
                    return crate::EventResponse::Consume;
                }
            }

            ApplicationEvent::MouseButton { state: ButtonState::Released, button: MouseButton::Left, position: (x, y) } => {
                if self.mouse_down_in_bounds && self.contains_point(*x, *y) {
                    if let Some(on_click) = &mut self.on_click {
                        message_queue.push((on_click)(state));
                    }
                    self.mouse_down_in_bounds = false;
                    return crate::EventResponse::Consume;
                }
                
                self.mouse_down_in_bounds = false;
            }

            _ => {}
        }

        crate::EventResponse::None
    }

    fn render_size(&self, _theme: &crate::style::Theme) -> (u32, u32) {
        self.bounds.dimensions()
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        renderer.draw(crate::render::DrawCommand::Circle {
            center: self.bounds.center(),
            radius: self.radius,
            color: self.color.unwrap(),
        });

        match &self.contents {
            Contents::Char(text) => {
                text.render(renderer, theme);
            }
            Contents::Image(image) => {
                image.render(renderer, theme);
            }
            Contents::None => {}
        }
    }
}