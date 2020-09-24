# **Surreal**: **S**imple **R**ust **U**ser **I**nterface **L**ibrary

## About
Surreal is a Rust-native UI library focused on simplicity and performance. Below is an example demonstrating the simple, declarative style of Surreal:
```Rust
use surreal::*;

// If you don't need messages, they can be left out. The message type will simply become `()`

#[derive(EmptyMessage)] // This trait prevents the message queue from accepting empty messages
#[empty(None)] // This attribute specifies which variant is "empty" (no message)
enum Message {
    UpdateCounterText,
    None,
}

fn main() {
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
                .message_handler(|this, message, mut state| {
                    if let Message::UpdateCounterText = message {
                        this.set_text(@counter.to_string());
                    }
                })
                // Fonts are specified by an alias (see `include_fonts!` below)
                .font("bold"),

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
                    })
                    // Buttons can be rounded using a percentage (0% to 100%)
                    .roundness(100.0),

                CircleButton::new("decrement")
                    // CircleButtons can contain either a character or an image resource
                    .character(Text::character('-'))
                    // If closures become too large, simply pass in a function
                    .on_click(|mut state| {
                        @counter -= 1;
                        Message::UpdateCounterText
                    }),
            },
        }
        // If the theme's default alignment isn't centered, this will make the VStack centered, but not change the nested HStack
        .alignment(Alignment::Center) 
    };

    // Generates a map of fonts with given aliases
    let fonts = include_fonts! {
        // "default" alias is required. All text will use this by default.
        default => "../default_font_path",
        bold    => "../bold_font_path",
    };

    // Window settings can be set using a builder pattern
    let app = Application::new(fonts)
        .resizable()
        .centered()
        .on_quit(|| println!("Quitting..."));

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

## Application Logic
Two interfaces exist for application logic. These can be used together, or alone.  
At the center of these are message--a user defined type sent out by various UI interactions.  

### Messages
Messages are sent by widgets like so:
```Rust
Widget::new("id")
    .on_some_event(|mut state| {
        @state_field.do_something();
        Message::ResponseToEvent
    }),
```

### Message Handlers
Message handlers can be attached to widgets. These accept the widget, the message to handle, and the application state:
```Rust
Widget::new("id")
    .message_handler(|this, message, mut state| {
        match message {
            MessageType::Variant { optional_data } => {
                this.do_something();
            }
            ...
        }
    }),
```

### View Hooks
View hooks are special in that they gain access to the view itself. This allows for element lookup by id, allowing hooks to effectively replace message handlers:
```Rust
let view = ...;

view.set_hook(|this, message| {
    match message {
        MessageType::Variant1 => {
            // Get a widget and cast it to the appropriate type
            let some_text: &mut Text<_> = get_widget_by_id(view, "text_id");
            some_text.set_text("Set from hook");
        }

        MessageType::Variant2 => {
            // Has access to the view
            this.do_something();
        }
        ...
    }
});
```

## Control Flow
TODO: Execution order
