use surreal::prelude::*;

#[derive(EmptyMessage)]
#[empty(None)]
enum Message {
    UpdateCounter,
    UpdateAmount,
    AppendButton,
    DeleteAppended,
    None,
}

#[allow(unused)]
pub fn main() {
    // TODO: Consider giving users the option of cloning `Shared<T>`s like 
    // gtk suggests (see `clone!`): https://gtk-rs.org/docs-src/tutorial/closures

    let mut view = Stateful! {
        @State {
            counter: i32 = 0,
            amount: u32 = 1,
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
                    // 1 <-> 50
                    @amount = 1 + (percentage * 49.0) as u32;
                    Message::UpdateAmount
                })
                .width(15)
                .container_color(Color::LIGHT_GRAY)
                .orientation(Orientation::Horizontal),

            Text::new("amount_text")
                .text(&@amount.to_string())
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
                Button::new("delete")
                    .text(Text::new("")
                        .text("Delete")
                        .color(Color::DARK_GRAY)
                    )
                    .on_click(|_| {
                        Message::DeleteAppended
                    }),
                
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

                Button::new("append")
                    .color(Color::LIGHT_GRAY)
                    .text(Text::new("")
                        .text("Append")
                        .color(Color::BLACK)
                    )
                    .on_click(|_| {
                        Message::AppendButton
                    }),

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
    view.set_hook(|view, message| {
        // Two ways to get widgets

        GetWidget!(view.more_text as Text)
        .set_text("Hook");

        GetWidget!(Button(reset) from view);

        match message {
            Message::AppendButton => {
                println!("Appending a button");
                view.append(
                    Button::new("_appended")
                        .text(Text::new("")
                            .text("Appended")
                            .scale(25.0)
                        )
                        .color(Color::new(0.5, 0.2, 0.2, 1.0))
                        .into_element()
                )
            }

            Message::DeleteAppended => {
                if view.delete("_appended").is_ok() {
                    println!("Deleting a button");
                } else {
                    println!("No appended buttons to delete")
                }
            }

            _ => {}
        }
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
        // TODO: see `application.rs` scrollbar todo
        // allow_scrollbars: true,
        fonts,
        images,
        ..Default::default()
    });
    
    app.run(&mut view);
}