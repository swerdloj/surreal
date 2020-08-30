use crate::state::{Shared, State, make_shared};
use crate::Orientation;
use crate::view_element::*;

#[derive(IntoViewElement)]
#[kind(View)]
pub struct Stack {
    orientation: Orientation,
    state: Option<Shared<State>>,
    // TODO: Children should be an ordered hashmap of (id -> element)
    // This would also enforce unique element ids
    children: Vec<ViewElement>,

    bounds: crate::bounding_rect::BoundingRect,
}

impl Stack {
    pub fn new(orientation: Orientation, children: Vec<ViewElement>) -> Self {
        Stack {
            orientation,
            state: None,
            children,

            bounds: crate::bounding_rect::BoundingRect::new(),
        }
    }
}

impl super::View for Stack {
    fn assign_state(&mut self, state: crate::state::State) {
        self.state = Some(make_shared(state));
    }

    // TODO: All child views should have clones of the root's `State` (and no `Option`)
    fn state(&self) -> Shared<State> {
        self.state.as_ref().unwrap().clone()
    }

    // TODO: Account for view padding
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
            match self.orientation {
                Orientation::Horizontal => current_x += theme.view_padding.horizontal,
                Orientation::Vertical => current_y += theme.view_padding.vertical,
            }

            match child {
                ViewElement::View(_view) => {
                    todo!();
                    // let (width, height) = view.size(text_renderer, theme);

                    // match self.orientation {
                    //     Orientation::Vertical => {
                    //         current_y += height;
                    //     }
                    //     Orientation::Horizontal => {
                    //         current_x += width;
                    //     }
                    // }
                }
                
                ViewElement::Widget(widget) => {
                    widget.place(current_x as i32, current_y as i32);
                    println!("Placing {} at ({}, {})", widget.id(), current_x, current_y);
                    
                    let (width, height) = widget.render_size(text_renderer, theme);
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

    fn render_width(&self) -> u32 {
        self.bounds.width
    }

    fn render_height(&self) -> u32 {
        self.bounds.height
    }

    fn children(&mut self) -> &mut Vec<crate::ViewElement> {
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
