use crate::state::State;

use std::cell::{RefCell, RefMut};

pub enum ViewElement {
    State(State),
    View(TestView),
    Widget(TestWidget),
    // Component(),
}

impl Into<ViewElement> for State {
    fn into(self) -> ViewElement {
        ViewElement::State(self)
    }
}
impl Into<ViewElement> for TestView {
    fn into(self) -> ViewElement {
        ViewElement::View(self)
    }
}
impl Into<ViewElement> for TestWidget {
    fn into(self) -> ViewElement {
        ViewElement::Widget(self)
    }
}

pub struct TestWidget {
    id: &'static str,
    pub state: i32,

    function: Option<Box<dyn Fn(RefMut<State>)>>,
}

impl TestWidget {
    pub fn new(id: &'static str) -> Self {
        TestWidget {
            id,
            state: 0,
            function: None,
        }
    }

    pub fn function<F: Fn(RefMut<State>) + 'static>(mut self, function: F) -> Self {
        self.function = Some(Box::new(function));
        self
    }

    pub fn call_function(&self, state: RefMut<State>) {
        if let Some(f) = &self.function {
            (f)(state);
        }
    }
}

pub struct TestView {
    pub state: RefCell<State>,
    // TODO: Replace Vec with ordered hashset (order is very important)
    children: Vec<ViewElement>,
}

impl TestView {
    pub fn new(state: State, children: Vec<ViewElement>) -> Self {
        TestView {
            state: RefCell::new(state), 
            children,
        }
    }

    // TODO: When this is a widget trait, how to upcast to proper widget type?
    pub fn get_widget_by_id(&self, id: &'static str) -> &TestWidget {
        for element in &self.children {
            match element {
                ViewElement::Widget(widget) => {
                    if widget.id == id {
                        return widget;
                    }
                }

                ViewElement::View(subview) => {
                    return subview.get_widget_by_id(id);
                }

                _ => {}
            }
        }

        panic!("Widget `{}` not found", id);
    }

    pub fn call_everything(&mut self) {
        for element in &mut self.children {
            match element {
                ViewElement::Widget(widget) => {
                    widget.call_function(self.state.borrow_mut());
                }

                _ => {}
            }
        }
    }

    pub fn call_widget_function_on(&mut self, id: &'static str) {
        let widget = self.get_widget_by_id(id);
        // widget.call_function(self.state.borrow_mut());
    }
}

#[macro_export]
macro_rules! TestView {
    ( $($e:expr),+ $(,)? ) => {{
        let mut state = State::new();
        let mut children = Vec::new();

        let mut has_state = false;

        $(
            let child = $e.into();
            match child {
                ViewElement::State(some_state) => {
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

        TestView::new(state, children)
    }};
}