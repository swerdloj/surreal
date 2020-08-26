use crate::state::{Shared, State};
use crate::{Orientation, ViewElement};

use std::rc::Rc;
use std::cell::RefCell;

pub struct Stack {
    orientation: Orientation,
    state: Shared<State>,
    // TODO: Children should be an ordered hashmap of (id -> element)
    // This would also enforce unique element ids
    children: Vec<ViewElement>,

    bounds: crate::bounding_rect::BoundingRect,
}

impl Stack {
    pub fn new(orientation: Orientation, state: State, children: Vec<ViewElement>) -> Self {
        Stack {
            orientation,
            state: Rc::new(RefCell::new(state)),
            children,

            bounds: crate::bounding_rect::BoundingRect::new(),
        }
    }
}

impl super::View for Stack {
    fn state(&self) -> Shared<State> {
        self.state.clone()
    }

    // TODO: Account for view padding
    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut view_width = 0;
        let mut view_height = 0;

        match self.orientation {
            Orientation::Horizontal => current_y += theme.padding.vertical,
            Orientation::Vertical => current_x += theme.padding.horizontal,
        }
        
        for child in &mut self.children {
            match self.orientation {
                Orientation::Horizontal => current_x += theme.padding.horizontal,
                Orientation::Vertical => current_y += theme.padding.vertical,
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

                ViewElement::TEMP_State(_) => unreachable!(),
            }
        }

        match self.orientation {
            Orientation::Vertical => {
                self.bounds.width = view_width + 2*theme.padding.horizontal;
                self.bounds.height = current_y + theme.padding.vertical;
            }
            Orientation::Horizontal => {
                self.bounds.width = current_x + theme.padding.horizontal;
                self.bounds.height = view_height + 2*theme.padding.vertical;
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

impl crate::IntoViewElement for Stack {
    fn into_element(self) -> ViewElement {
        ViewElement::View(Box::new(self))
    }
}

// TODO: Create a third "Stack" macro that V/HStack forward to
// by supplying the expressions and the varient.
// This prevents the two macros from having duplicated bodies.
// Eventually, a procedural macro would replace the need for this


#[macro_export]
macro_rules! VStack {
    ( $($component:expr),+ $(,)? ) => {{
        let mut state = State::new();
        let mut children = Vec::new();

        let mut has_state = false;

        $(
            let child = $component.into_element();
            match child {
                ViewElement::TEMP_State(some_state) => {
                    if has_state {
                        panic!("State can only be declared once per view");
                    } else {
                        state = some_state;
                        has_state = true;
                    }
                }

                _ => children.push(child),
            }
        )+

        Stack::new(Orientation::Vertical, state, children)
    }};
}

#[macro_export]
macro_rules! HStack {
    ( $($component:expr),+ $(,)? ) => {{
        let mut state = State::new();
        let mut children = Vec::new();

        let mut has_state = false;

        $(
            let child = $component.into_element();
            match child {
                ViewElement::TEMP_State(some_state) => {
                    if has_state {
                        panic!("State can only be declared once per view");
                    } else {
                        state = some_state;
                        has_state = true;
                    }
                }

                _ => children.push(child),
            }
        )+

        Stack::new(Orientation::Horizontal, state, children)
    }};
}
