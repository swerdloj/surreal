pub mod button;
pub mod text;

pub trait Widget : crate::IntoViewElement {
    fn id(&self) -> &'static str;
}