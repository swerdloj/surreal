pub mod stack;

pub use stack::Stack;

pub type ViewHook<Msg> = fn(&mut dyn View<Msg>, &Msg);

// TODO: Integrate this. Also see if `get_by_id` can work for this type
// struct ViewMap<Msg: crate::EmptyMessage> {
//     // Map of element id to a sequence of child indices
//     index_sequence: std::collections::HashMap<&'static str, Vec<usize>>,
//     // Ordered list of child elements
//     pub children: Vec<crate::ViewElement<Msg>>,
// }

// VStack, HStack, ListView, and more can all be created using just the `Stack` struct,
// but other views may be desired such as TabView, GridView, ScrollView, and so on
pub trait View<Msg: crate::EmptyMessage> {
    fn state(&self) -> crate::state::Shared<crate::state::State>;
    /// Assigns the state to all views in the view tree
    fn assign_state(&mut self, state: crate::state::Shared<crate::state::State>);

    // TODO: See ViewMap
    // fn map(&mut self) -> &mut ViewMap<Msg>;
    // fn children(&mut self) -> &mut Vec<crate::ViewElement<Msg>> {
    //     &mut self.map().children
    // }

    fn children(&mut self) -> &mut Vec<crate::ViewElement<Msg>>;

    fn translate(&mut self, dx: i32, dy: i32);

    /// This function is called when views are initialized. Use this to implement the theme defaults.
    fn init(&mut self, _renderer: &mut crate::render::Renderer, _theme: &crate::style::Theme) {

    }

    // FIXME: Is there any way to prevent this from being replaced?
    // FIXME: The naming of this and `init` is dangerous
    /// The default init function for views. Do not implement this; use `View::init()` instead
    fn _init(&mut self, renderer: &mut crate::render::Renderer, theme: &crate::style::Theme) {
        self.init(renderer, theme);
        for child in self.children() {
            match child {
                crate::ViewElement::Widget(widget) => {
                    widget.init(renderer, theme);
                }
                crate::ViewElement::View(view) => {
                    view._init(renderer, theme);
                }
            }
        }
    }

    fn layout(&mut self, renderer: &mut crate::render::Renderer, theme: &crate::style::Theme, constraints: (u32, u32), is_root: bool);

    fn render_width(&self) -> u32;
    fn render_height(&self) -> u32;
    fn render_size(&self) -> (u32, u32) {
        (self.render_width(), self.render_height())
    }

    // TODO: Should views serve only as containers?
    // Implementing this as part of the trait will not allow otherwise.
    // Might want to allow backgrounds or outlines for views.
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
            }
        }
    }

    fn propogate_event(&mut self, event: &sdl2::event::Event, message_queue: &mut crate::MessageQueue<Msg>) {
        use crate::ViewElement::*;

        let state = self.state();
        for child in self.children() {
            match child {
                View(view) => {
                    view.propogate_event(event, message_queue);
                }

                Widget(widget) => {
                    widget.handle_event(event, state.clone().borrow_mut(), message_queue);
                }
            }
        }
    }

    // Returns true if the view should resize
    fn propogate_message(&mut self, message: &Msg) -> bool {
        let state = self.state();

        let mut should_resize = false;

        for child in self.children() {
            match child {
                crate::ViewElement::View(view) => {
                    should_resize |= view.propogate_message(message);
                }

                crate::ViewElement::Widget(widget) => {
                    widget.handle_message(message, state.clone().borrow_mut());
                    
                    should_resize |= widget.check_if_should_resize_then_reset_to_false();
                }
            }
        }

        should_resize
    }

    // FIXME: I want hook to be FnMut, but I can only do this if I require the
    // function to come in here as Box<FnMut>.
    // This would be fixed via generic parameters, but traits don't allow that.
    fn set_hook(&mut self, hook: ViewHook<Msg>);
    fn get_hook(&self) -> Option<&ViewHook<Msg>>;
}

pub(crate) fn call_hook<Msg: crate::EmptyMessage>(view: &mut dyn View<Msg>, message: &Msg) {
    if let Some(hook) = view.get_hook() {
        (hook)(view, message);
    }
}

/// Returns `None` if the widget was not found
pub fn get_widget_by_id<'a, T: crate::widget::Widget<Msg>, Msg: crate::EmptyMessage>(view: &'a mut dyn View<Msg>, id: &str) -> Option<&'a mut Box<T>> {
    for child in view.children() {
        match child {
            crate::ViewElement::Widget(widget) => {
                if widget.id() == id {
                    unsafe {
                        // TODO: Add a type-check to ensure safety such as `Widget::type`
                        return Some(&mut *(widget as *mut Box<dyn crate::widget::Widget<Msg>> as *mut Box<T>));
                    }
                }
            }

            crate::ViewElement::View(view) => {
                let result = get_widget_by_id::<T, Msg>(&mut (**view), id);
                if result.is_some() {
                    return result;
                }
            }
        }
    }

    None
}

#[macro_export]
macro_rules! GetWidget {
    ($ty:ident($id:ident) from $view:expr) => {
        get_widget_by_id::<$ty<_>, _>($view, stringify!($id))
            .expect(&format!("No such widget `{}`", stringify!($id)))
    };

    ($view:ident.$id:ident as $ty:ident) => {
        get_widget_by_id::<$ty<_>, _>($view, stringify!($id))
            .expect(&format!("No such widget `{}`", stringify!($id)))
    };
}