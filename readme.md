# **SRUIL**: **S**imple **R**ust **U**ser **I**nterface **L**ibrary <br> aka "Surreal" 

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
        // This is the id of the VStack
        ("root_view")

        // This becomes a variable accessible by anything that comes after via `vars`
        counter: i32,
        
        Text::new("counter_text")
            .on_update(|text, vars| text = vars.counter.to_string())
            .update_animation(Animations::Bump),

        HStack! {
            // This is the id of the HStack
            ("button_row")

            Button::new("increment")
                .text("+")
                .on_click(|vars| vars.counter += 1),

            Button::new("decrement")
                .text("-")
                .on_click(|vars| vars.counter -= 1)
        }
    };

    app.run(view);
}
```
