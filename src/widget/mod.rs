pub mod button;
pub mod text;

pub trait Widget : crate::IntoViewElement {
    fn id(&self) -> &'static str;

    fn render(&self, render_target: &mut crate::application::RenderTarget, gpu: &mut crate::application::gpu, text_renderer: &mut crate::font::TextRenderer);
}