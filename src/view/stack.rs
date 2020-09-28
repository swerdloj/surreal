use crate::state::{Shared, State};
use crate::{Orientation, Alignment};
use crate::view_element::*;

pub struct Stack<Msg: EmptyMessage> {
    orientation: Orientation,
    alignment: Option<Alignment>,
    state: Option<Shared<State>>,
    children: Vec<ViewElement<Msg>>,

    hook: Option<super::ViewHook<Msg>>,

    bounds: crate::bounding_rect::BoundingRect,
}

impl<Msg: EmptyMessage> Stack<Msg> {
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

impl<Msg: EmptyMessage> super::View<Msg> for Stack<Msg> where Msg: 'static{
    fn assign_state(&mut self, state: Shared<State>) {
        for child in &mut self.children {
            if let ViewElement::View(view) = child {
                view.assign_state(state.clone());
            }
        }

        self.state = Some(state);
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

    // TODO: Utilize alignment (create helper functions for each alignment?)
    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme, is_root: bool) {
        let mut current_x = 0;
        let mut current_y = 0;
        let mut view_width = 0;
        let mut view_height = 0;

        // Initial padding within window
        if is_root {
            current_x += theme.view_padding.horizontal;
            current_y += theme.view_padding.vertical;
        }
        
        for child in &mut self.children {
            let child_width;
            let child_height;

            match child {
                ViewElement::View(view) => {
                    view.layout(text_renderer, theme, false);
                    let size = view.render_size();
                    child_width = size.0; 
                    child_height = size.1;
                    
                    view.translate(current_x as i32, current_y as i32);
                }
                
                ViewElement::Widget(widget) => {
                    widget.place(current_x as i32, current_y as i32);
                    // println!("Placing {} at ({}, {})", widget.id(), current_x, current_y);
                    
                    let size = widget.render_size(theme);
                    child_width = size.0; 
                    child_height = size.1;
                }
            }

            // Determines size of view thus far & where to place next element
            match self.orientation {
                Orientation::Vertical => {
                    current_y += child_height + theme.widget_padding.vertical;
                    view_width = std::cmp::max(child_width, view_width);
                }
                Orientation::Horizontal => {
                    current_x += child_width + theme.widget_padding.horizontal;
                    view_height = std::cmp::max(child_height, view_height);
                }
            }
        }

        // NOTE: Subtraction is to remove the extra padding added in final iteration of the above loop
        if is_root { // Add extra padding at the end of the root view (to match the initial padding)
            match self.orientation {
                Orientation::Vertical => {
                    self.bounds.width = current_x + view_width + theme.view_padding.horizontal;
                    self.bounds.height = current_y + theme.view_padding.vertical - theme.widget_padding.vertical;
                }
                Orientation::Horizontal => {
                    self.bounds.width = current_x + theme.view_padding.horizontal - theme.widget_padding.horizontal;
                    self.bounds.height = current_y + view_height + theme.view_padding.vertical;
                }
            }
        } else { // Do not add any more padding --> treat child view just like a widget
            match self.orientation {
                Orientation::Vertical => {
                    self.bounds.width = view_width;
                    self.bounds.height = current_y - theme.widget_padding.vertical;
                }
                Orientation::Horizontal => {
                    self.bounds.width = current_x - theme.widget_padding.horizontal;
                    self.bounds.height = view_height;
                }
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