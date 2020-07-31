#[macro_use]
extern crate surreal;

use surreal::application::Application;
use surreal::state::State;
use surreal::view::stack::Stack;
use surreal::{IntoViewElement, Orientation, ViewElement};
use surreal::widget::button::Button;

fn main() {
    let mut view = VStack! {
        State! {
            test: u32 = 0,
        },

        Button::new("test")
            .on_click(|mut state| {
                let test = state.get::<u32>("test");
                *test += 1;
                
                println!("Presses: {}", test);
            }),

        Button::new("test2"),
    };

    let app = Application::new("Test", 800, 600);
    app.run(&mut view);
}