use crate::view_element::*;
use super::Widget;

pub struct Image<Msg> {
    id: &'static str,
    resource: String,
    pub bounds: crate::bounding_rect::BoundingRect,
    width_constraint: Option<u32>,
    height_constraint: Option<u32>,
    should_resize: bool,
    _phantom_marker: std::marker::PhantomData<Msg>,
}

impl<Msg> Image<Msg> {
    pub fn new(id: &'static str) -> Self {
        Self {
            id,
            resource: String::from(""),
            bounds: crate::bounding_rect::BoundingRect::new(),
            width_constraint: None,
            height_constraint: None,
            should_resize: false,
            _phantom_marker: std::marker::PhantomData,
        }
    }

    pub fn resource(mut self, alias: &str) -> Self {
        self.resource = String::from(alias);
        self
    }

    pub fn fit_to_width(mut self, width: u32) -> Self {
        self.width_constraint = Some(width);
        self
    }

    pub fn fit_to_height(mut self, height: u32) -> Self {
        self.height_constraint = Some(height);
        self
    }
}

impl<Msg: EmptyMessage + 'static> Widget<Msg> for Image<Msg> {
    fn id(&self) -> &'static str {
        self.id
    }

    fn should_resize(&mut self) -> &mut bool {
        &mut self.should_resize
    }

    fn init(&mut self, text_renderer: &mut crate::render::font::TextRenderer, theme: &crate::prelude::Theme) {
        if self.resource == "" {
            panic!("Image `{}` was never assigned a resource. Use `.resource('resource_alias')` to assign this.");
        }

        // TODO: Do something to load/store the image & set bounds
        todo!()
    }

    fn place(&mut self, x: i32, y: i32) {
        todo!()
    }

    fn translate(&mut self, dx: i32, dy: i32) {
        todo!()
    }

    fn render_size(&self, /*text_renderer: &mut crate::render::font::TextRenderer,*/ theme: &crate::prelude::Theme) -> (u32, u32) {
        todo!()
    }

    fn render(&self, renderer: &mut crate::render::ContextualRenderer, theme: &crate::prelude::Theme) {
        todo!()
    }
}