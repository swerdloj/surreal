#[macro_use]
extern crate surreal;

use surreal::prelude::*;

fn main() {
    // TODO: See whether state can be shared between widgets somehow
    // using `FnMut` closures and `Rc<RefCell>`
    // Consider an auto-cloning macro with a `Shared<T>` type

    // TODO: Implement an Elm-like message system (and replace `State`?)
    let mut view = VStack! {
        State! {
            test: u32 = 0,
        },

        Button::new("text_button")
            .text(Text::new("") // <-- Nested id is not needed
                .text("Button......")
                .scale(40.0)
                .color(Color::BLACK)
            )
            .style(PrimitiveStyle::RoundedRectangle {
                roundness: 100.0,
            }),

        Text::new("text")
            .text("This is a text widget with some text in it")
            .scale(30.0),

        Button::new("test")
            .on_click(|mut state| {
                let test = state.get::<u32>("test");
                *test += 1;

                println!("Presses: {}", test);
            })
            .text(Text::new("")
                .text("+")
                .scale(60.0)
            )
            .color(Color::LIGHT_GRAY),

        Text::new("more_text")
            .text("More text here")
            .color(Color::new(0.4, 0.6, 0.8, 1.0))
            .scale(50.0),

        Button::new("test2")
            .style(PrimitiveStyle::Rectangle),

        Button::new("test3"),
        
        Button::new("test4")
            .color(Color::new(0.2, 0.3, 0.8, 1.0)),
    };

    // NOTE: The extra '../' here is because this is in the '/bin' folder
    let fonts = include_fonts! {
        default => "../../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
    };

    // TODO: This + pass to app
    // let theme = include_theme!("../../res/themes/default.theme")

    let mut app = Application::new(ApplicationSettings {
        title: "Surreal Test",
        fonts,
        ..Default::default()
    });

    app.run(&mut view);
}