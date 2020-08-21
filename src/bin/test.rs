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

    // TODO: Consider implementing an Elm-like message system
    let mut view = VStack! {
        State! {
            test: u32 = 0,
        },

        Button::new("above_text"),

        Text::new("text")
            .text("This is a text widget with some text in it")
            .scale(30.0),

        Button::new("test")
            .on_click(|mut state| {
                let test = state.get::<u32>("test");
                *test += 1;

                println!("Presses: {}", test);
            })
            .color(Color::DARK_GRAY),

        Text::new("more_text")
            .text("More text here")
            .color(Color::new(0.4, 0.6, 0.8, 1.0))
            .scale(50.0),

        Button::new("test2"),
        Button::new("test3")
            .color(Color::new(0.8, 0.1, 0.1, 1.0)),
    };

    // NOTE: The extra '../' here is because this is in the '/bin' folder
    let fonts = include_fonts! {
        default => "../../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
    };

    // TODO: This + pass to app
    // let theme = include_theme!("../../res/themes/default.theme")

    let mut app = Application::new("Test", 800, 600, fonts)
        .fit_window_to_view(true);

    app.run(&mut view);
}