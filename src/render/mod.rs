/*
TODO:

This module will be for everything related to rendering.
This includes quads and shaders.

The idea is to create a basic canvas-like API for rendering.
Example of what this would look like:

// Example 1
Button::render(render_target, ...) {
    render_target.rounded_rect(RoundedRect {
        at: self.rect.position,
        size: self.rect.size,
        color: Color::Red,
        border: Color::Black,
        border_width: 5,
    });
}

// Example 2
Thing::render(render_target, ...) {
    render_target.circle(Circle {
        radius: 10,
        ...
    });

    render_target.rect(Rect {
        ...
    });
}

*/

macro_rules! size_of {
    // Size of type
    ($T:ty) => {
        std::mem::size_of::<$T>() as _
    };
    
    // (Dynamic) Size of pointed-to value
    (var $I:ident) => {
        std::mem::size_of_val(&$I) as _
    };
}


pub mod font;
pub mod quad;


pub fn screen_space_to_draw_space(point: (i32, i32), window_dimensions: (u32, u32)) -> (f32, f32) {
    //let aspect_ratio = window_dimensions.0 as f32 / window_dimensions.1 as f32;

    (
        ((point.0 as f32 / window_dimensions.0 as f32) - 0.5) * 2.0,// * aspect_ratio,
        ((point.1 as f32 / window_dimensions.1 as f32) - 0.5) * 2.0
    )
}


pub enum DrawCommand {
    Text(glyph_brush::OwnedSection),
    Rect {
        top_left: (i32, i32),
        width: u32,
        height: u32,
        color: crate::Color,
    },
}

/// Bundles Renderer with required context for an easy-to-use construct.
///
/// This is done to avoid needing to pass a context type to all `Renderer` functions 
/// (and to all view element render-functions by extension).
pub struct ContextualRenderer<'frame> {
    pub renderer: &'frame mut Renderer,

    pub device: &'frame wgpu::Device,
    pub target: &'frame wgpu::TextureView,
    pub encoder: &'frame mut wgpu::CommandEncoder,
    pub window_dimensions: (u32, u32),
}

impl<'frame> ContextualRenderer<'frame> {
    pub fn draw(&mut self, command: DrawCommand) {
        self.renderer.draw(
            command, 
            self.device,
            self.target,
            self.encoder,
            self.window_dimensions,
        );
    }

    pub fn get_font_id(&self, alias: &str) -> wgpu_glyph::FontId {
        self.renderer.text_renderer.get_font_id(alias)
    }
}

/// Contains functionality for rendering to a target
pub struct Renderer {
    // Quad essentially serves as a brush with relevant information being
    // passed through vertices/uniforms
    quad: quad::Quad,

    // quad_bind_group_layout: wgpu::BindGroupLayout,
    quad_render_pipeline: wgpu::RenderPipeline,

    pub text_renderer: font::TextRenderer,
}

// Reference: https://github.com/hecrj/iced/blob/master/wgpu/src/

impl Renderer {
    pub fn new(device: &wgpu::Device, text_renderer: font::TextRenderer) -> Self {
        let quad_bind_group_layout = crate::render::quad::Quad::bind_group_layout(device);
        let quad_render_pipeline = crate::render::quad::Quad::create_render_pipeline(device, &quad_bind_group_layout, crate::TEXTURE_FORMAT);
        
        // Default -- will be adjusted by render calls
        let quad = quad::Quad::new(
            device,
            &quad_bind_group_layout,
            (1, 1),
            (0, 0),
            0,
            0,
        );

        Self {
            quad,
            // quad_bind_group_layout,
            quad_render_pipeline,
            text_renderer,
        }
    }

    pub fn draw_text(&mut self, device: &wgpu::Device, encoder: &mut wgpu::CommandEncoder, frame: &wgpu::TextureView, window_dimensions: (u32, u32), section: glyph_brush::OwnedSection) {
        self.text_renderer.render_section(
            device, 
            frame, 
            encoder, 
            window_dimensions.0, 
            window_dimensions.1, 
            section,
        );
    }

    pub fn draw(&mut self, command: DrawCommand, device: &wgpu::Device, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder, window_dimensions: (u32, u32)) {
        match command {
            DrawCommand::Rect { top_left, width, height, color } => {
                self.quad.update_vertices(device, window_dimensions, top_left, width, height);
                self.quad.update_uniforms(
                    device, 
                    encoder,
                    quad::Uniforms {
                    color: color.as_array().into(),
                });

                let mut render_pass = Self::create_render_pass(encoder, target);

                render_pass.set_pipeline(&self.quad_render_pipeline);
                self.quad.render(&mut render_pass);
            }

            DrawCommand::Text(section) => {
                self.draw_text(device, encoder, target, window_dimensions, section);
            }
        }
    }

    fn create_render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, target: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Load,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: crate::Color::CLEAR.into(),
                },
            ],
            depth_stencil_attachment: None,
        })
    }
}