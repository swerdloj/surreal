#[macro_use]
extern crate surreal;

// TODO: Create a simple prelude for users to important whatever is needed
use surreal::application::Application;
use surreal::state::State;
use surreal::view::stack::Stack;
use surreal::{IntoViewElement, Orientation, ViewElement};
use surreal::widget::button::Button;
use surreal::widget::text::Text;
use surreal::bounding_rect::BoundingRect;
use surreal::Color;

fn main() {
    // TODO: See whether state can be shared between widgets somehow
    // using FnMut closures and Rc<RefCell>

    let mut view = VStack! {
        State! {
            test: u32 = 0,
        },

        Text::new("text")
            .text("This is a text widget with some text in it")
            .scale(30.0),

        Button::new("test")
            .on_click(|mut state| {
                let test = state.get::<u32>("test");
                *test += 1;

                println!("Presses: {}", test);
            })
            .bounds(BoundingRect {
                x: 10,
                y: 150,
                width: 200,
                height: 100,
            })
            .color(Color::ALMOST_WHITE),

        Button::new("test2")
            .bounds(BoundingRect {
                x: 10,
                y: 260,
                width: 780,
                height: 100,
            })
            .color(Color::DARK_GRAY),
    };

    // NOTE: The extra '../' here is because this is in the '/bin' folder
    let fonts = include_fonts! {
        default => "../../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
    };

    // TODO: This + pass to app
    // let theme = include_themes! { 
    //     default => "../../res/themes/default.theme"
    // }

    let mut app = Application::new("Test", 800, 600, fonts);
    app.run(&mut view);
}