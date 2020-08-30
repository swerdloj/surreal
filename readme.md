# **Surreal**: **S**imple **R**ust **U**ser **I**nterface **L**ibrary

## About
Surreal is a Rust-native UI library focused on simplicity and extensibility. Below is an example demonstrating the minimalist nature of Surreal:
```Rust
use surreal::*;

fn main() {
    // Window settings can be set using a builder pattern
    let app = Application::new()
        .resizable()
        .centered()
        .vsync(false)
        .on_quit(|| println!("Quitting..."));

    // Views use declarative syntax
    // The `Stateful!` macro adds custom syntax for working with shared state
    let mut view = Stateful! {
        // Procedural macro magic happens here
        @State {
            counter: i32 = 0,
        },

        // `@counter` is compiled to `state.get::<i32>("counter")`
        VStack! { 
            Text::new("counter_text")
                .on_update(|text, state| text = @counter.to_string())
                .update_animation(Animations::Bump),

            HStack! {
                Button::new("increment")
                    .text("+")
                    .on_click(|state| @counter += 1),

                Button::new("decrement")
                    .text("-")
                    .on_click(|state| @counter -= 1),
            },
        }
    };

    app.run(&mut view);
}
```
