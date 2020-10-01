use surreal::prelude::*;

#[derive(EmptyMessage)]
#[empty(None)]
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
            Image::new("image")
                .resource("public_domain")
                .fit_to_width(150),

            Button::new("text_button")
                .text(Text::new("") // <-- Nested id is not needed
                    .text("Button")
                    .color(Color::BLACK)
                )
                .roundness(100.0),

            Text::new("text")
                .text("This is a text widget with text inside")
                .scale(30.0),

            HStack! {
                // TODO: CircleButton with image should automatically scale
                // the image's largest dimension to the radius and account for
                // internal padding
                CircleButton::new("with_image")
                    .image(Image::new("")
                        .resource("plus")
                        .fit_to_width(70)
                    )
                    .radius(40)
                    .color(Color::LIGHT_GRAY)
                    .on_click(|mut state| {
                        @counter += 1;
                        Message::UpdateCounter
                    }),

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
            },

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
                .text(Text::character('+')
                    .scale(70.0)
                    .into()
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

                Button::new("reset")
                    .color(Color::DARK_GRAY)
                    .text(Text::new("")
                        .text("Reset")
                        .color(Color::new(0.9, 0.2, 0.3, 1.0))
                        .scale(35.0)
                    )
                    .on_click(|mut state| {
                        @counter = 0;
                        Message::UpdateCounter
                    }),
            },

            Button::new("test7"),

            Button::new("test8"),
        }
        .alignment(Alignment::Center),
    };

    // NOTE: Future: Define view via DSL. Hook can then be used to implement the view
    view.set_hook(|view, _message| {
        let test: &mut Text<_> = get_widget_by_id(view, "more_text").unwrap();
        test.set_text("Hook");

        // let text: &mut Text<_> = view.get_widget_by_id("more_text");
    });

    // NOTE: The extra '../' here is because this is in the '/bin' folder
    let fonts = include_fonts! {
        default => "../../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
    };

    let images = include_images! {
        public_domain => "../../res/images/public_domain.png",
        plus => "../../res/images/plus_thing.png",
    };

    // TODO: This + pass to app
    // let theme = include_theme!("../../res/themes/default.theme");

    let mut app = Application::new(ApplicationSettings {
        title: "Surreal Test",
        fonts,
        images,
        ..Default::default()
    });

    app.run(&mut view);
}