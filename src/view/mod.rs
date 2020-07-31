pub mod stack;

// VStack, HStack, ListView, and more can all be created using just the `Stack` struct,
// but other views may be desired such as TabView, GridView, ScrollView, and so on
pub trait View : crate::IntoViewElement {
    fn children(&mut self) -> &mut Vec<crate::ViewElement>;
}