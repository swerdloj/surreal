#[macro_use]
extern crate surreal;

use surreal::state::State;
use surreal::view::{TestView, TestWidget, ViewElement};

#[derive(Debug)]
struct CustomType<T> {
    field: T,
}

impl<T: Default> CustomType<T> {
    fn new() -> Self {
        CustomType {
            field: T::default()
        }
    }
}

#[derive(Debug)]
enum SomeEnum {
    Yes,
    No,
}

fn main() {
    let mut state = State::new();

    let mut state2 = State! {
        test: i32 = -1,
        generic: CustomType<f32> = CustomType::new(),
        enum: SomeEnum = SomeEnum::No,
    };

    let test_i32 = state2.get_i32("test");
    *test_i32 = 7;

    assert_eq!(7, *state2.get::<i32>("test"));

    println!("State macro: {:?}", state2);

    // TODO: Remaining tests
    //       1) Access & modify state from multiple widgets
    //       2) Access & modify widgets from other widgets by id
    let mut view = TestView! {
        State! {
            something: i32 = 0,
        },

        TestWidget::new("test_1"),

        TestView! {
            TestWidget::new("test_2")
                .function(|mut state| {
                    println!("This function works");

                    let thing = state.get::<i32>("something");
                    *thing += 2;

                    println!("{}", state.get::<i32>("something"));
                }),
        },
    };

    view.call_widget_function_on("test_2");
    
    let widget = view.get_widget_by_id("test_2");
    widget.call_function(view.state.borrow_mut());
    

    // println!("View macro: {:?}", view);
}
