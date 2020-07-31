use super::Widget;

pub struct Text {
    id: &'static str,
    text: String,
}

impl Text {

}

impl Widget for Text {
    fn id(&self) -> &'static str {
        self.id
    }
}

impl crate::IntoViewElement for Text {
    fn as_element(self) -> crate::ViewElement {
        crate::ViewElement::Widget(Box::new(self))
    }
}