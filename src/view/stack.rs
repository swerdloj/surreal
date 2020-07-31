use crate::state::State;
use crate::{Orientation, ViewElement};

use std::cell::RefCell;

pub struct Stack {
    orientation: Orientation,
    state: RefCell<State>,
    // TODO: Children should be an ordered hashmap of (id -> element)
    // This would also enforce unique element ids
    children: Vec<ViewElement>,
}

impl Stack {
    pub fn new(orientation: Orientation, state: State, children: Vec<ViewElement>) -> Self {
        Stack {
            orientation,
            state: RefCell::new(state),
            children,
        }
    }
}

impl super::View for Stack {
    fn children(&mut self) -> &mut Vec<crate::ViewElement> {
        &mut self.children
    }
}

impl crate::IntoViewElement for Stack {
    fn as_element(self) -> ViewElement {
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
            let child = $component.as_element();
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
        
        Stack {
            orientation: Orientation::Horizontal,
            ...
        }
    }};
}
