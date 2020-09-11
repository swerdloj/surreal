#[macro_use]
extern crate surreal;

use surreal::prelude::*;

fn main() {
    // TODO: Consider giving users the option of cloning `Shared<T>`s like 
    // gtk suggests (see `clone!`): https://gtk-rs.org/docs-src/tutorial/closures


    // TODO: Implement an Elm-like message system for cross-element communication
    let mut view = Stateful! {
        @State {
            counter: u32 = 0,
        },

        VStack! {
            Button::new("text_button")
                .text(Text::new("") // <-- Nested id is not needed
                    .text("Button")
                    .scale(40.0)
                    .color(Color::BLACK)
                )
                .style(PrimitiveStyle::RoundedRectangle {
                    roundness: 100.0,
                }),

            Text::new("text")
                .text("This is a text widget with text inside")
                .scale(30.0),

            Button::new("test")
                .on_click(|mut state| {
                    @counter += 1;
                    println!("Presses: {}", @counter);
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

            HStack! {
                Button::new("test3"),
                
                Button::new("test4")
                    .color(Color::new(0.2, 0.3, 0.8, 1.0))
                    .text(Text::new("")
                        .text("state")
                        .scale(35.0)
                    )
                    .on_click(|mut state| {
                        println!("{}", @counter);
                    }),

                Button::new("test5")
                    .color(Color::LIGHT_GRAY),
                Button::new("test_")
                    .color(Color::LIGHT_GRAY),
            },

            Button::new("test6"),
        },
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