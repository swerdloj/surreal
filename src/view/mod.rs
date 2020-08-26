pub mod stack;

pub use stack::Stack;

// VStack, HStack, ListView, and more can all be created using just the `Stack` struct,
// but other views may be desired such as TabView, GridView, ScrollView, and so on
pub trait View : crate::IntoViewElement {
    fn state(&self) -> crate::state::Shared<crate::state::State>;

    fn children(&mut self) -> &mut Vec<crate::ViewElement>;

    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme);

    fn render_width(&self) -> u32;
    fn render_height(&self) -> u32;

    // TODO: Should views serve only as containers?
    // Implementing this as part of the trait will not allow otherwise
    // After all, views simply arrange widgets & elements
    fn render(&mut self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        use crate::ViewElement::*;
        for child in self.children() {
            match child {
                View(view) => {
                    view.render(renderer, theme);
                }
                
                Widget(widget) => {
                    widget.render(renderer, theme);
                }
                
                TEMP_State(_state) => {
                    // TEMP: This will be removed eventually
                }
            }
        }

        // FIXME: All text needs to be drawn at the same time
        // This function should be called only one time per frame
        // This is so wgpu_glyph can cache the text, meaning this call should not be made inside `View`
        // Using individual draw calls per `Section` raises CPU usage from <1% to >5% (>22% in debug build)
        renderer.renderer.text_renderer.render_queue(renderer.device, renderer.target, renderer.encoder, renderer.window_dimensions.0, renderer.window_dimensions.1);
    }

    fn propogate_event(&mut self, event: &sdl2::event::Event) {
        use crate::ViewElement::*;

        let state = self.state();
        for child in self.children() {
            match child {
                View(view) => {
                    view.propogate_event(event);
                }

                Widget(widget) => {
                    widget.handle_event(event, state.clone().borrow_mut());
                }

                _ => {}
            }
        }
    }
}