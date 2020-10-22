// TODO: Go through and decide what needs to be pub, pub(crate), & private
pub mod application;

pub mod style;
pub mod animation;

pub mod state;
pub mod view;
pub mod widget;
pub mod component;

pub mod render;
pub mod bounding_rect;

pub mod timing;


// Desktop format
#[cfg(all(not(target_arch = "wasm32"), any(target_os = "linux", target_os = "windows", target_os = "macos")))]
pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;
// Web format
#[cfg(target_arch = "wasm32")]
pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;
// Android format
#[cfg(target_os = "android")]
pub const TEXTURE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;


/// Re-exports everything needed by users for easy library import via `use surreal::prelude::*;`
pub mod prelude {
    pub use crate::{
        surreal_macros::*,
        style::{Theme, DEFAULT_THEME, PrimitiveStyle},
        application::{Application, ApplicationSettings},
        state,
        widget::*,
        view::*,
        render::font::IncludedFonts,
        Color,
        EventResponse,
        Alignment,
        Orientation,
        ViewElement,
        IntoViewElement,
        EmptyMessage,
    };
}

/// Re-exports all macros related to surreal
pub mod surreal_macros {
    // Procedural macros
    pub use proc_macros::{
        Stateful, 
        EmptyMessage,
    };

    // Regular macros
    pub use crate::{
        VStack, 
        HStack, 
        State,
        GetWidget,
        include_fonts,
        include_images,
    };
}


// Custom events used by Surreal. These events replace the need for
// the overly complex, inflexible, and lacking winit events.
// TODO: Consider just wrapping winit events and include mouse info
pub mod event {
    pub enum MouseButton {
        Left,
        Right,
        Middle,
        Other(u8),
    }

    pub enum ButtonState {
        Pressed,
        Released,
    }

    pub enum ApplicationEvent {
        MouseMotion {
            position: (i32, i32),
            relative_change: (i32, i32),
        },

        MouseButton {
            state: ButtonState,
            button: MouseButton,
            position: (i32, i32),
        },

        None,
    }

    impl ApplicationEvent {
        pub fn is_none(&self) -> bool {
            if let ApplicationEvent::None = self {true} else {false}
        }
    }
}



/// Types and derive macro required when using `#derive(IntoViewElement)`
pub mod view_element {
    pub use crate::{IntoViewElement, ViewElement, EmptyMessage};
}

pub trait EmptyMessage {
    fn is_message(&self) -> bool;
}

// Allow views that don't use messages
impl EmptyMessage for () {
    fn is_message(&self) -> bool {
        false
    }
}

pub struct MessageQueue<Msg> {
    queue: Vec<Msg>,
}

impl<Msg: EmptyMessage> MessageQueue<Msg> {
    fn new() -> Self {
        Self { queue: Vec::new() }
    }

    /// Add a message to the message queue, ignoring empty messages
    pub fn push(&mut self, message: Msg) {
        if message.is_message() {
            self.queue.push(message);
        }
    }

    /// Empties the message queue. Return iterator over the contents.
    fn drain(&mut self) -> std::vec::Drain<Msg> {
        self.queue.drain(..)
    }
}

pub enum EventResponse {
    /// Event will be consumed, preventing it from propogating any further.
    ///
    /// For example, a button will `Consume` left click events, 
    /// preventing other widgets from seeing that event.
    Consume,
    /// Signal the renderer to redraw the view. Does not consume the event.
    Redraw,
    /// No response given. The event will propogate to other elements.
    None,
}

/// f32 RGBA color
// TODO: Consider making a `Color!` macro that accepts RGB, RGBA, hex, etc.
// TODO: Should these be f64, or does it not matter?
#[derive(Copy, Clone)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const AUBERGINE:    Color = Color { r: 0.03,  g: 0.0,  b: 0.02,  a: 1.0 };
    pub const ALMOST_WHITE: Color = Color { r: 0.9,   g: 0.9,  b: 0.9,   a: 1.0 };
    pub const BLACK:        Color = Color { r: 0.0,   g: 0.0,  b: 0.0,   a: 1.0 };
    pub const CLEAR:        Color = Color { r: 0.0,   g: 0.0,  b: 0.0,   a: 0.0 };
    pub const DARK_GRAY:    Color = Color { r: 0.01,  g: 0.01, b: 0.01,  a: 1.0 };
    pub const LIGHT_GRAY:   Color = Color { r: 0.3,   g: 0.3,  b: 0.3,   a: 1.0 };
    pub const WHITE:        Color = Color { r: 1.0,   g: 1.0,  b: 1.0,   a: 1.0 };

    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {r, g, b, a}
    }
    
    pub fn from<T: Into<[f32; 4]>>(color: T) -> Self {
        let [r, g, b, a] = color.into();
        Self::new(r, g, b, a)
    }

    pub fn as_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

impl Into<cgmath::Vector4<f32>> for Color {
    fn into(self) -> cgmath::Vector4<f32> {
        cgmath::Vector4::new(self.r, self.g, self.b, self.a)
    }
}

impl Into<wgpu::Color> for Color {
    fn into(self) -> wgpu::Color {
        wgpu::Color { r: self.r as f64, g: self.g as f64, b: self.b as f64, a: self.a as f64 }
    }
}

/// Element alignment
#[derive(Copy, Clone)]
pub enum Alignment {
    Left,
    Right,
    Center,
}

impl Alignment {
    pub fn is_left_aligned(&self) -> bool {
        if let Alignment::Left = self {true} else {false}
    }
    pub fn is_right_aligned(&self) -> bool {
        if let Alignment::Right = self {true} else {false}
    }
    pub fn is_centered(&self) -> bool {
        if let Alignment::Center = self {true} else {false}
    }
}

/// Element orientation
pub enum Orientation {
    Vertical,
    Horizontal,
}

impl Orientation {
    pub fn is_vertical(&self) -> bool {
        if let Orientation::Vertical = self {true} else {false}
    }
    pub fn is_horizontal(&self) -> bool {
        if let Orientation::Horizontal = self {true} else {false}
    }
}

pub enum ViewElement<Msg: EmptyMessage> {
    Widget(Box<dyn crate::widget::Widget<Msg>>),
    View(Box<dyn crate::view::View<Msg>>),
}

// TODO: Make sure the blanket impl hack doesn't break anything
pub trait IntoViewElement<Msg: EmptyMessage, ImplConstraint> {
    fn into_element(self) -> ViewElement<Msg>;
}

// FIXME: These types are leaked by view macros (but not imported through prelude)
pub struct __WidgetBlanket;
pub struct __ViewBlanket;

// Unit struct hack from: https://jsdw.me/posts/rust-fn-traits/
impl<Msg: EmptyMessage, T: widget::Widget<Msg> + 'static> IntoViewElement<Msg, __WidgetBlanket> for T {
    fn into_element(self) -> ViewElement<Msg> {
        ViewElement::Widget(Box::new(self))
    }
}

impl<Msg: EmptyMessage, T: view::View<Msg> + 'static> IntoViewElement<Msg, __ViewBlanket> for T {
    fn into_element(self) -> ViewElement<Msg> {
        ViewElement::View(Box::new(self))
    }
}


//////////// TESTING /////////////

struct Hack1;struct Hack2;struct Hack3;struct Hack4;struct Hack5;

trait InsertViewElement<Msg: EmptyMessage, ImplConstraint> {
    fn insert_elements(self, list: &mut Vec<ViewElement<Msg>>);
}

// Void type insert nothing
impl<Msg: EmptyMessage> InsertViewElement<Msg, Hack1> for () {
    fn insert_elements(self, _list: &mut Vec<ViewElement<Msg>>) {
        // nothing
    }
}

// Widgets and Views insert themselves
impl<Msg: EmptyMessage, T: IntoViewElement<Msg, __WidgetBlanket>> InsertViewElement<Msg, Hack2> for T {
    fn insert_elements(self, list: &mut Vec<ViewElement<Msg>>) {
        list.push(self.into_element())
    }
}
impl<Msg: EmptyMessage, T: IntoViewElement<Msg, __ViewBlanket>> InsertViewElement<Msg, Hack3> for T {
    fn insert_elements(self, list: &mut Vec<ViewElement<Msg>>) {
        list.push(self.into_element())
    }
}

// Lists of Widgets and Lists of Views insert their contents
impl<Msg: EmptyMessage, T: IntoViewElement<Msg, __WidgetBlanket>> InsertViewElement<Msg, Hack4> for Vec<T> {
    fn insert_elements(self, list: &mut Vec<ViewElement<Msg>>) {
        for item in self.into_iter() {
            list.push(item.into_element())
        }
    }
}
impl<Msg: EmptyMessage, T: IntoViewElement<Msg, __ViewBlanket>> InsertViewElement<Msg, Hack5> for Vec<T> {
    fn insert_elements(self, list: &mut Vec<ViewElement<Msg>>) {
        for item in self.into_iter() {
            list.push(item.into_element())
        }
    }
}

macro_rules! TestStack {
    ( $($component:expr),+ $(,)? ) => {{
        let mut children = Vec::new();

        $(
            ($component).insert_elements(&mut children);
        )+

        Stack::new(Orientation::Vertical, children)
    }};
}

// TODO: Support multiple expressions and an else case
macro_rules! Conditional {
    ( $cond:expr => $block:expr ) => {
        if $cond {
            vec![$block]
        } else {
            vec![]
        }
    };
}

// TEMP: Here for testing purposes
#[allow(unused)]
fn test() {
    use prelude::*;

    // TODO: See if boxing within the macros rather than within IntoViewElement
    // can allow for all 3 cases
    // TODO: Create an `if` macro that returns an empty vec if no else block is given

    let empty1: Vec<Button<()>> = Vec::new();
    let empty2 = ();

    let list: Vec<Box<dyn IntoViewElement<(), _>>> = vec![
        Box::new(Button::new("01")),
        Box::new(Text::new("02")),
    ];

    let condition = true;

    let view = TestStack! {
        // Base case
        Button::new("1"),

        // Nested case
        TestStack! {
            CircleButton::new("2"),
            Text::new("3"),
            empty2,
        },

        // List case
        // list,

        Conditional! {
            condition => Button::new("test")
        },

        // Empty "if" case(s?)
        empty1,
        empty2,
    };
}