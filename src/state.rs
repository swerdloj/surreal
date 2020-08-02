use std::collections::HashMap;
use std::any::Any;

// use std::cell::{RefCell, RefMut};

macro_rules! make_get_type {
    ( $($t:ty), + ) => {
        paste::paste! {
            $(
                #[allow(non_snake_case)]
                pub fn [<get_ $t>](&mut self, id: &'static str) -> &mut $t {
                    self.get::<$t>(id)
                }
            )+
        }
    };
}

#[derive(Debug)]
pub struct State {
    vars: HashMap<&'static str, Box<dyn Any>>,
}

impl State {
    // TODO: Remove this once procedural macro is made. 
    // This is a dirty duck-typing macro hack
    pub fn as_element(self) -> crate::ViewElement {
        crate::ViewElement::TEMP_State(self)
    }

    pub fn new() -> Self {
        State {
            vars: HashMap::new(),
        }
    }

    pub fn add_var(&mut self, id: &'static str, var: Box<dyn Any>) {
        if let Some(_old) = self.vars.insert(id, var) {
            panic!("A variable with id `{}` already exists", id);
        }
    }

    fn get_any_mut(&mut self, id: &'static str) -> &mut Box<dyn Any> {
        if let Some(var) = self.vars.get_mut(id) {
            var
        } else {
            panic!("No such variable exists: `{}`", id);
        }
    }
    
    fn get_any(&self, id: &'static str) -> &Box<dyn Any> {
        if let Some(var) = self.vars.get(id) {
            var
        } else {
            panic!("No such variable exists: `{}`", id);
        }
    }

    pub fn get<T: 'static>(&mut self, id: &'static str) -> &mut T {
        if let Some(var) = self.get_any_mut(id).downcast_mut::<T>() {
            var
        } else {
            panic!("Downcast of `{}` failed. Double check its type.", id);
        }
    }

    pub fn peek<T: 'static>(&self, id: &'static str) -> &T {
        if let Some(var) = self.get_any(id).downcast_ref::<T>() {
            var
        } else {
            panic!("Downcast of `{}` failed. Double check its type.", id);
        }
    }

    // Convenience getters for primitive types
    make_get_type!(
        u8, u16, u32, u64, u128, 
        i8, i16, i32, i64, i128, 
        f32, f64,
        isize, usize,
        bool,
        char,
        String
    );
}

// TODO: It might be worth storing the type information somewhere since it
//       is required anyway. This can be used for debugging (and perhaps even
//       for an in-app variable editor).
#[macro_export]
macro_rules! State {
    ( $($name:ident : $type:ty = $value:expr),+ $(,)? ) => {{
        let mut state = surreal::state::State::new();

        $(
            // Force proper type (don't want "12" to become an i32 when meant for u32)
            let __temp: $type = $value;
            state.add_var(stringify!($name), Box::new(__temp));
        )+

        state
    }};
}