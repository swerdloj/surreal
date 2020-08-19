pub mod button;
pub mod text;

pub trait Widget : crate::IntoViewElement {
    fn id(&self) -> &'static str;

    fn handle_event(&mut self, _event: &sdl2::event::Event) -> crate::EventResponse {
        crate::EventResponse::None
    }

    fn render_size(&self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) -> (u32, u32);
    fn place(&mut self, x: i32, y: i32);

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme);
}