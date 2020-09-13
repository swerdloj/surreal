# **Surreal**: **S**imple **R**ust **U**ser **I**nterface **L**ibrary

## About
Surreal is a Rust-native UI library focused on simplicity and performance. Below is an example demonstrating the simple, declarative style of Surreal:
```Rust
use surreal::*;

// If you don't need messages, they can be left out. The message type will simply become `()`
enum Message {
    UpdateCounterText,
    None,
}

fn main() {
    // Window settings can be set using a builder pattern
    let app = Application::new()
        .resizable()
        .centered()
        .on_quit(|| println!("Quitting..."));

    // Views use declarative syntax
    // The `Stateful!` macro adds custom syntax for working with shared state
    let mut view = Stateful! {
        // Procedural macro magic happens here
        @State {
            counter: i32 = 0,
        },

        // `@counter` is compiled to `*state.get::<i32>("counter")`
        VStack! { 
            Text::new("counter_text")
                // Message handlers have access to their owners such as this very Text widget
                .message_handler(|mut this, message, mut state| {
                    if let Message::UpdateCounterText = message {
                        this.set_text(@counter.to_string());
                    }
                }),

            HStack! {
                Button::new("increment")
                    // Note that nested widgets don't need ids, but they can help with debugging
                    .text(Text::new("") 
                            .text("+")
                        )
                    // Elements can send messages to communicated with other elements
                    .on_click(|mut state| {
                        @counter += 1;
                        Message::UpdateCounterText
                    }),

                Button::new("decrement")
                    .text(Text::new("")
                        .text("-")
                    )
                    // If closures become too large, simply pass in a function
                    .on_click(|mut state| {
                        @counter -= 1;
                        Message::UpdateCounterText
                    }),
            },
        }.alignment(Alignment::Center) // If the theme's default alignment isn't centered, this will make the VStack centered, but not change the nested HStack
    };

    app.run(&mut view);
}
```

## Customizability
Surreal offers two means of customizing the appearance of an application:
### Global Themes
A global theme defines the default style for all view elements. This includes colors, shapes, fonts, padding, and layouts.

Themes are declared using struct-like syntax:
```Rust
@Theme: "Example theme"

colors: Colors {
    primary: Color::DARK_GRAY // Buttons default to this color
    secondary: Color::ALMOST_WHITE
    ...
},

text: Text {
    scale: 50.0, // Text will default to this size
    ...
}

```

### Widget Style
The global theme can be overriden on a per-element basis. Each view element features builder functions for changing style. For example, the following will ignore the global theme's button color and text scale:
```Rust
Button::new("using_specific_color")
    .color(Colors::WHITE) // Would otherwise use theme's button color
    .text(Text::new("")
        .text("This text has a specific size")
        .scale(100.0) // Would otherwise use theme's text scale
    )
```