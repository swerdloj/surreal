pub mod stack;

pub use stack::Stack;

pub type ViewHook<Msg> = fn(&mut dyn View<Msg>, &Msg);

// VStack, HStack, ListView, and more can all be created using just the `Stack` struct,
// but other views may be desired such as TabView, GridView, ScrollView, and so on
pub trait View<Msg> : crate::IntoViewElement<Msg> {
    fn state(&self) -> crate::state::Shared<crate::state::State>;
    /// Assigns the root view's state
    fn assign_state(&mut self, state: crate::state::State);
    /// Assign a view's state by sharing a parent's state
    fn share_state(&mut self, state: crate::state::Shared<crate::state::State>);

    fn children(&mut self) -> &mut Vec<crate::ViewElement<Msg>>;

    fn translate(&mut self, dx: i32, dy: i32);

    /// This function is called when views are initialized. Use this to implement the theme defaults.
    fn init(&mut self, _text_renderer: &mut crate::render::font::TextRenderer, _theme: &crate::style::Theme) {

    }

    // FIXME: Is there any way to prevent this from being replaced?
    // FIXME: The naming of this and `init` is dangerous
    /// The default init function for views. Do not implement this; use `View::init()` instead
    fn _init(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme) {
        self.init(text_renderer, theme);
        for child in self.children() {
            match child {
                crate::ViewElement::Widget(widget) => {
                    widget.init(text_renderer, theme);
                }
                crate::ViewElement::View(view) => {
                    view._init(text_renderer, theme);
                }
            }
        }
    }

    fn layout(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::style::Theme);

    fn render_width(&self) -> u32;
    fn render_height(&self) -> u32;
    fn render_size(&self) -> (u32, u32) {
        (self.render_width(), self.render_height())
    }

    fn should_resize(&mut self) -> bool {
        let mut should_resize = false;
        for child in self.children() {
            match child {
                crate::ViewElement::Widget(widget) => {
                    let should = widget.should_resize();
                    should_resize = should_resize || *should;
                    *should = false;
                }
                crate::ViewElement::View(view) => {
                    should_resize = should_resize || view.should_resize();
                }
            }
        }

        should_resize
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

    fn propogate_message(&mut self, message: &Msg) {
        let state = self.state();

        for child in self.children() {
            match child {
                crate::ViewElement::View(view) => {
                    view.propogate_message(message);
                }

                crate::ViewElement::Widget(widget) => {
                    widget.handle_message(message, state.clone().borrow_mut());
                }
            }
        }
    }

    // FIXME: I want hook to be FnMut, but I can only do this if I require the
    // function to come in here as Box<FnMut>.
    // This would be fixed via generic parameters, but traits don't allow that.
    fn set_hook(&mut self, hook: ViewHook<Msg>);
    fn get_hook(&self) -> Option<&ViewHook<Msg>>;

    // FIXME: This is what I want, not `call_hook`
    // fn call_hook(&mut self, message: &Msg) where Msg: 'static {
    //     if let Some(hook) = self.get_hook() {
    //         (hook)(self, message);
    //     }
    // }
}

// TODO: Make this work (requires &'a to be static for some reason)
pub trait ViewIntrospection<Msg>{
    fn get_widget_by_id<'a, T: crate::widget::Widget<Msg>>(&'a mut self, id: &str) -> &'a mut Box<T>;
}

impl<Msg> ViewIntrospection<Msg> for dyn View<Msg> {
    fn get_widget_by_id<'a, T: crate::widget::Widget<Msg>>(&'a mut self, id: &str) -> &'a mut Box<T> {
        for child in self.children() {
            match child {
                crate::ViewElement::Widget(widget) => {
                    if widget.id() == id {
                        unsafe {
                            // TODO: Add a type-check to ensure safety such as `Widget::type`
                            return std::mem::transmute::<&mut Box<dyn crate::widget::Widget<Msg>>, &mut Box<T>>(widget);
                        }
                    }
                }
    
                crate::ViewElement::View(view) => {
                    return view.get_widget_by_id::<T>(id);
                }
            }
        }
    
        panic!("No widget with id `{}` exists", id);
    }
}

pub fn call_hook<Msg>(view: &mut dyn View<Msg>, message: &Msg) {
    if let Some(hook) = view.get_hook() {
        (hook)(view, message);
    }
}

pub fn get_widget_by_id<'v, T: crate::widget::Widget<Msg>, Msg>(view: &'v mut dyn View<Msg>, id: &str) -> &'v mut T {
    for child in view.children() {
        match child {
            crate::ViewElement::Widget(widget) => {
                if widget.id() == id {
                    unsafe {
                        // TODO: Add a type-check to ensure safety such as `Widget::type`
                        return std::mem::transmute::<&mut Box<dyn crate::widget::Widget<Msg>>, &mut Box<T>>(widget);
                    }
                }
            }

            crate::ViewElement::View(view) => {
                // FIXME: Is this correct?
                return get_widget_by_id::<T, Msg>(&mut (**view), id);
            }
        }
    }

    panic!("No widget with id `{}` exists", id);
}