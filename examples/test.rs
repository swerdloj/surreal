use surreal::prelude::*;

#[derive(EmptyMessage)]
#[empty(None)]
enum Message {
    UpdateCounter,
    UpdateAmount,
    None,
}

#[allow(unused)]
pub fn main() {
    // TODO: Consider giving users the option of cloning `Shared<T>`s like 
    // gtk suggests (see `clone!`): https://gtk-rs.org/docs-src/tutorial/closures

    let mut view = Stateful! {
        @State {
            counter: i32 = 0,
            amount: u32 = 0,
        },

        VStack! {
            // Image::new("image")
            //     .resource("public_domain")
            //     .fit_to_width(150),

            Button::new("text_button")
                .text(Text::new("") // <-- Nested id is not needed
                    .text("Button")
                    .color(Color::BLACK)
                )
                .roundness(100.0),

            Text::new("text")
                .text("This is a text widget with text inside")
                .scale(30.0),

            ScrollBar::new("scroll_test", 200, 50)
                .on_scroll(|percentage, mut state| {                    
                    // 0 <-> 50
                    @amount = (percentage * 50.0) as u32;
                    Message::UpdateAmount
                })
                .width(15)
                .container_color(Color::LIGHT_GRAY)
                .orientation(Orientation::Horizontal),

            Text::new("amount_text")
                .text("0")
                .scale(25.0)
                .message_handler(|this, message, mut state| {
                    if let Message::UpdateAmount = message {
                        this.set_text(&format!("{}", @amount));
                    } 
                })
                .color(Color::WHITE),

            HStack! {
                // Contents are automatically scaled to fit within the CircleButton
                CircleButton::new("with_image")
                    .image(Image::new("")
                        .resource("plus")
                    )
                    .radius(40)
                    .color(Color::LIGHT_GRAY)
                    .on_click(|mut state| {
                        @counter += @amount as i32;
                        Message::UpdateCounter
                    }),

                CircleButton::new("circle_button")
                    .character(Text::character('-')
                        .color(Color::BLACK)
                        .scale(100.0)
                    )
                    .radius(40)
                    .color(Color::LIGHT_GRAY)
                    .on_click(|mut state| {
                        @counter -= @amount as i32;
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
        }
        .alignment(Alignment::Center),
    };

    // NOTE: Future: Can define view via DSL. Hook can then be used to implement the view
    view.set_hook(|view, _message| {
        // Two ways to get widgets

        let test = GetWidget!(view.more_text as Text);
        test.set_text("Hook");

        let _test2 = GetWidget!(Button(reset) from view);
    });

    // Resources are embeded by default. They can be loaded from disk instead
    // if the feature "embed-resources" is disabled
    let fonts = include_fonts! {
        default => "../res/JetBrainsMono/JetBrainsMono-Medium.ttf",
    };

    let images = include_images! {
        public_domain => "../res/images/public_domain.png",
        plus => "../res/images/plus_thing.png",
    };

    // TODO: Load & pass themes
    let mut app = Application::new(ApplicationSettings {
        title: "Surreal Test",
        // prevents locking window min size to main-view's size
        allow_scrollbars: true, 
        fonts,
        images,
        ..Default::default()
    });
    
    app.run(&mut view);
}