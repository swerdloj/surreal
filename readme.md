# **Surreal**: **S**imple **R**ust **U**ser **I**nterface **L**ibrary

## Note

Surreal originated as a research project. The corresponding paper is currently in process.

Surreal exists to explore declarative user interface design patterns.  
As it stands, Surreal is usable, but lacks many features.  

Please see the **Project** tab for a list of todos. There are also many such todos throughout the Surreal codebase as well as notes for future work.

For a working example, see `examples/test.rs` which demonstrates all of Surreal's current capabilities.

## About
Surreal is a Rust-native user interface library focused on simplicity and performance. Below is an example demonstrating the simple, declarative style of Surreal:
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
                    // Elements can send messages to communicate with other elements
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

    // Specify window and application settings
     let app = Application::new(ApplicationSettings {
        title: "Surreal Example",
        fonts,
        ..Default::default()
    });

    app.run(&mut view);
}
```

### Building
Surreal currently works with the following platforms:
- Windows
- Linux
- Android
- Web (unstable)
- Mac (untested)
- iOS (untested)

**Android Builds**  
How to build `wgpu-rs` + `winit` apps for Android:

1. Install the [sdkmanager](https://developer.android.com/studio/command-line/sdkmanager) tool. See [this](https://stackoverflow.com/a/60598900) for help
2. Install the build tools and platform tools using `sdkmanager`
3. Install a compatible SDK and NDK version using `sdkmanager`
4. Install `cargo-apk` via `cargo install cargo-apk`
5. Follow the [android-ndk-rs](https://github.com/rust-windowing/android-ndk-rs) guide to configure your application's Android build (also see `./examples/android.rs`)
6. Build the app using `cargo apk build (your build flags here)`
7. You may get various warnings. Check `your_app/target/debug/apk` for `your_app.apk` before taking any action
8. Sign your app using the following JDK commands:
    - ensure your JDK path is exported (mine is C:\Program Files\Java\jdk-10\bin)
    - `keytool -genkey -v -keystore my-release-key.keystore -alias alias_name -keyalg RSA -keysize 2048 -validity 10000`
      - This command does not need to be used each time
    - `jarsigner -verbose -sigalg SHA1withRSA -digestalg SHA1 -keystore my-release-key.keystore your_app.apk alias_name`  
      
    [You can also look into apksigner.jar](https://developer.android.com/studio/publish/app-signing.html#signing-manually) located in `your_android_path/build-tools/your_version/lib`

9. Connect your developer-enabled device and run `adb install "path/to/your_app.apk"`
10. View console output with `adb logcat RustStdoutStderr:D *:S` (will also output panics, etc.)

 

## Customizability
Surreal offers two means of customizing the appearance of an application:
### Global Themes
A global theme defines the default style for all view elements. This includes colors, shapes, fonts, padding, and layouts.

See `src/style.rs` for available styling options.

TODO: Themes are declared using struct-like syntax:
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
    .on_some_action(|mut state| {
        @state_field.do_something();
        Message::ResponseToAction
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
View hooks are special in that they gain access to the view itself. This allows for element lookup by id, allowing hooks to effectively replace message handlers.

Additionally, hooks can be used to append and delete widgets within views:
```Rust
let view = ...;

view.set_hook(|this, message| {
    match message {
        MessageType::Variant1 => {
            // Get a widget and cast it to the appropriate type
            GetWidget!(view.text_id as Text);
                .set_text("Set from hook");

            // Has access to the view
            this.do_something();
        }

        MessageType::Variant2 => {
            // Append a widget to the view
            this.append(Widget::new("id"));
            // Remove widget from view
            this.delete("id");
        }
        ...
    }
});
```

## Control Flow (TODO: Outdated)
- **Initialization**:
  - View::_init -> View::init + Widget::init -> View::layout
- **Main Loop**:
  - **Event Loop** (generates messages):
    - View::propogate_event -> Widget::handle_event
  - **Message Loop** (signals resize):
    - View::call_hook -> View::propogate_message -> Widget::handle_message
  - **If should_resize**:
    - repeat Initialization
  - **Render**:
    - View::render + Widget::render