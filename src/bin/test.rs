#[macro_use]
extern crate surreal;

use surreal::prelude::*;

enum Message {
    UpdateCounter,
    None,
}

fn main() {
    // TODO: Consider giving users the option of cloning `Shared<T>`s like 
    // gtk suggests (see `clone!`): https://gtk-rs.org/docs-src/tutorial/closures

    let mut view = Stateful! {
        @State {
            counter: i32 = 0,
        },

        VStack! {
            Button::new("text_button")
                .text(Text::new("") // <-- Nested id is not needed
                    .text("Button")
                    .color(Color::BLACK)
                )
                .roundness(100.0),

            Text::new("text")
                .text("This is a text widget with text inside")
                .scale(30.0),

            CircleButton::new("circle_button")
                .color(Color::LIGHT_GRAY)
                .radius(40)
                .character(Text::character('-')
                    .color(Color::BLACK)
                    .scale(100.0)
                )
                .on_click(|mut state| {
                    @counter -= 1;

                    Message::UpdateCounter
                }),

            Text::new("counter_text")
                .text("Counter: 0")
                .scale(50.0)
                .message_handler(|this, message, mut state| {
                    if let Message::UpdateCounter = message {
                        this.set_text(&format!("Counter: {}", @counter));
                    }
                }),

            Button::new("test")
                .on_click(|mut state| {
                    @counter += 1;

                    Message::UpdateCounter
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
                .roundness(0.0),

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

                        Message::None
                    }),

                Button::new("test5")
                    .color(Color::LIGHT_GRAY),
                Button::new("test6")
                    .color(Color::LIGHT_GRAY),
            },

            Button::new("test7"),
        }
        .alignment(Alignment::Center),
    };

    // TODO: Create a new type for view introspection that exposes only what users need
    // Accessing elements by id
    // Adding/removing elements
    // Adjust view parameters (padding, alignment, etc.)
    view.set_hook(|view, _message| {
        let test: &mut Text<_> = get_widget_by_id(view, "more_text");
        test.set_text("Hook");

        // TODO: Get this to work (see `test()` below for better compiler errors)
        // let test: &mut Text<_> = view.get_widget_by_id("more_text");

        println!("This is from a hook with access to the view");
    });

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

fn test(view: &mut dyn View<Message>) {
    let test: &mut Text<_> = view.get_widget_by_id("more_text");
}