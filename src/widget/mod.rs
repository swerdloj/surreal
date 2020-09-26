mod button;
mod circle_button;
mod text;
mod image;

pub use image::Image;
pub use circle_button::CircleButton;
pub use button::Button;
pub use text::Text;

pub trait Widget<Msg> : crate::IntoViewElement<Msg> where Msg : crate::EmptyMessage {
    fn id(&self) -> &'static str;

    fn handle_event(&mut self, _event: &sdl2::event::Event, _state: std::cell::RefMut<crate::state::State>, _message_queue: &mut crate::MessageQueue<Msg>) -> crate::EventResponse {
        crate::EventResponse::None
    }

    fn handle_message(&mut self, _message: &Msg, _state: std::cell::RefMut<crate::state::State>) {

    }

    /// Checks whether the widget requested resize, then resets the widget's should_resize state to false.
    fn check_if_should_resize_then_reset_to_false(&mut self) -> bool {
        let should = self.should_resize();
        let clone = *should;

        *should = false;

        clone
    }

    fn should_resize(&mut self) -> &mut bool;

    fn init(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme);

    fn place(&mut self, x: i32, y: i32);
    fn translate(&mut self, dx: i32, dy: i32);
    
    fn render_size(&self, /*text_renderer: &mut crate::render::font::TextRenderer,*/ theme: &crate::style::Theme) -> (u32, u32);
    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme);
}