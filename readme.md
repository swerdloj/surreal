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
        .vsync(false);

    // Views use declarative syntax
    let view = VStack! { 
        // This becomes a variable accessible by anything that comes after via `state`
        State! {
            counter: i32 = 0,
        },
        
        Text::new("counter_text")
            .on_update(|text, state| text = state.get::<i32>("counter").to_string())
            .update_animation(Animations::Bump),

        HStack! {
            Button::new("increment")
                .text("+")
                .on_click(|state| state.get_i32("counter") += 1),

            Button::new("decrement")
                .text("-")
                .on_click(|state| state.get_i32("counter") -= 1)
        },
    };

    app.run(view);
}
```
