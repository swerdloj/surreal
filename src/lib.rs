pub mod state;
pub mod view;
pub mod application;
pub mod widget;
pub mod rectangle;

use crate::widget::Widget;
use crate::view::View;

pub enum Orientation {
    Vertical,
    Horizontal,
}

// TODO: State will be removed once procedural macro is implemented
pub enum ViewElement {
    Widget(Box<dyn Widget>),
    View(Box<dyn View>),
    
    #[allow(non_camel_case_types)]
    TEMP_State(crate::state::State),
}

// TODO: Create derive macro for this
pub trait IntoViewElement {
    fn as_element(self) -> ViewElement;
}