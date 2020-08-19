pub mod stack;

// VStack, HStack, ListView, and more can all be created using just the `Stack` struct,
// but other views may be desired such as TabView, GridView, ScrollView, and so on
pub trait View : crate::IntoViewElement {
    fn children(&mut self) -> &mut Vec<crate::ViewElement>;

    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme);

    fn render_width(&self) -> u32;
    fn render_height(&self) -> u32;

    // TODO: Should views serve only as containers?
    // Implementing this as part of the trait will not allow otherwise
    fn render(&mut self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::style::Theme) {
        for child in self.children() {
            use crate::ViewElement::*;
            match child {
                View(view) => {
                    view.render(renderer, theme);
                }
                
                Widget(widget) => {
                    widget.render(renderer, theme);
                }
                
                TEMP_State(_state) => {
                    // This will be removed eventually
                }
            }
        }
    }
}