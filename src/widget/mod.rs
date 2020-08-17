pub mod button;
pub mod text;

pub trait Widget : crate::IntoViewElement {
    fn id(&self) -> &'static str;

    fn handle_event(&mut self, _event: &sdl2::event::Event) -> crate::EventResponse {
        crate::EventResponse::None
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer);
}