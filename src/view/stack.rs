use crate::state::{Shared, State, make_shared};
use crate::{Orientation, Alignment};
use crate::view_element::*;

#[derive(IntoViewElement)]
#[kind(View)]
pub struct Stack<Msg> {
    orientation: Orientation,
    alignment: Option<Alignment>,
    state: Option<Shared<State>>,
    // TODO: Children should be an ordered hashmap of (id -> element)
    // This would also enforce unique element ids
    children: Vec<ViewElement<Msg>>,

    hook: Option<super::ViewHook<Msg>>,

    bounds: crate::bounding_rect::BoundingRect,
}

impl<Msg> Stack<Msg> {
    pub fn new(orientation: Orientation, children: Vec<ViewElement<Msg>>) -> Self {
        Stack {
            orientation,
            alignment: None,
            state: None,
            children,
            hook: None,
            bounds: crate::bounding_rect::BoundingRect::new(),
        }
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = Some(alignment);
        self
    }
}

impl<Msg> super::View<Msg> for Stack<Msg> where Msg: 'static{
    fn share_state(&mut self, state: Shared<State>) {
        self.state = Some(state);
    }

    fn assign_state(&mut self, state: crate::state::State) {
        let shared_state = make_shared(state);

        for child in &mut self.children {
            match child {
                ViewElement::View(view) => {
                    view.share_state(shared_state.clone());
                }

                _ => {}
            }
        }
        self.state = Some(shared_state);
    }

    fn init(&mut self, _text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {
        if let Some(_alignment) = &self.alignment {} 
        else {
            self.alignment = Some(theme.default_alignment);
        }
    }

    // TODO: All child views should have clones of the root's `State` (and no `Option`)
    fn state(&self) -> Shared<State> {
        self.state.as_ref().unwrap().clone()
    }

    fn set_hook(&mut self, hook: super::ViewHook<Msg>) {
        self.hook = Some(hook);
    }

    fn get_hook(&self) -> Option<&super::ViewHook<Msg>> {
        self.hook.as_ref()
    }

    // TODO: Differentiate view & widget padding
    // TODO: Utilize alignment
    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut view_width = 0;
        let mut view_height = 0;

        match self.orientation {
            Orientation::Horizontal => current_y += theme.view_padding.vertical,
            Orientation::Vertical => current_x += theme.view_padding.horizontal,
        }
        
        for child in &mut self.children {
            let width;
            let height;

            match self.orientation {
                Orientation::Horizontal => current_x += theme.view_padding.horizontal,
                Orientation::Vertical => current_y += theme.view_padding.vertical,
            }

            match child {
                ViewElement::View(view) => {
                    view.layout(text_renderer, theme);
                    let size = view.render_size();
                    width = size.0; 
                    height = size.1;
                    
                    // TODO: Adjust view_width & view_height
                    // FIXME: Should not need to account for padding here
                    match self.orientation {
                        Orientation::Vertical => {
                            view.translate((current_x - theme.view_padding.horizontal) as i32, current_y as i32);
                        }
                        Orientation::Horizontal => {
                            todo!()
                        }
                    }
                }
                
                ViewElement::Widget(widget) => {
                    widget.place(current_x as i32, current_y as i32);
                    // println!("Placing {} at ({}, {})", widget.id(), current_x, current_y);
                    
                    let size = widget.render_size(theme);
                    width = size.0; 
                    height = size.1;
                }
            }

            match self.orientation {
                Orientation::Vertical => {
                    current_y += height;
                    view_width = std::cmp::max(width, view_width);
                }
                Orientation::Horizontal => {
                    current_x += width;
                    view_height = std::cmp::max(height, view_height);
                }
            }
        }

        match self.orientation {
            Orientation::Vertical => {
                self.bounds.width = view_width + 2*theme.view_padding.horizontal;
                self.bounds.height = current_y + theme.view_padding.vertical;
            }
            Orientation::Horizontal => {
                self.bounds.width = current_x + theme.view_padding.horizontal;
                self.bounds.height = view_height + 2*theme.view_padding.vertical;
            }
        }
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        self.bounds.x += dx;
        self.bounds.y += dy;

        for child in &mut self.children {
            match child {
                ViewElement::Widget(widget) => {
                    widget.translate(dx, dy);
                }
                ViewElement::View(view) => {
                    view.translate(dx, dy);
                }
            }
        }
    }

    fn render_width(&self) -> u32 {
        self.bounds.width
    }

    fn render_height(&self) -> u32 {
        self.bounds.height
    }

    fn children(&mut self) -> &mut Vec<crate::ViewElement<Msg>> {
        &mut self.children
    }
}

// TODO: Create a third "Stack" macro that is called by V/HStack
// with the proper orientation.
// This prevents the two macros from having duplicated bodies.


/// Builds a view with `Orientation::Vertical` 
#[macro_export]
macro_rules! VStack {
    ( $($component:expr),+ $(,)? ) => {{
        let mut children = Vec::new();

        $(
            let child = $component.into_element();
            children.push(child);
        )+

        Stack::new(Orientation::Vertical, children)
    }};
}

/// Builds a view with `Orientation::Horizontal` 
#[macro_export]
macro_rules! HStack {
    ( $($component:expr),+ $(,)? ) => {{
        let mut children = Vec::new();

        $(
            let child = $component.into_element();
            children.push(child);
        )+

        Stack::new(Orientation::Horizontal, children)
    }};
}