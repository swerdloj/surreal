pub mod button;
pub mod text;

pub trait Widget : crate::IntoViewElement {
    fn id(&self) -> &'static str;

    fn handle_event(&mut self, _event: &sdl2::event::Event) -> crate::EventResponse {
        crate::EventResponse::None
    }

    fn render(&self, render_context: &mut crate::application::RenderContext, gpu: &mut crate::application::gpu, text_renderer: &mut crate::font::TextRenderer);
}