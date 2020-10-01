/// Helper macro for obtaining
/// - Size of `Type` -> `size_of(Type)`
/// - Size of `variable` -> `size_of(var variable)`
macro_rules! size_of {
    ($T:ty) => {
        std::mem::size_of::<$T>() as _
    };
    
    // (Dynamic) Size of pointed-to value
    (var $I:ident) => {
        std::mem::size_of_val(&$I) as _
    };
}


pub mod font;
pub mod texture;
mod quad;


mod primitive {
    pub const RECTANGLE: u32 = 0;
    pub const ROUNDED_RECTANGLE: u32 = 1;
    pub const CIRCLE: u32 = 2;
}


pub fn screen_space_to_draw_space(point: (i32, i32), window_dimensions: (u32, u32)) -> (f32, f32) {
    //let aspect_ratio = window_dimensions.0 as f32 / window_dimensions.1 as f32;

    (
        ((point.0 as f32 / window_dimensions.0 as f32) - 0.5) * 2.0,// * aspect_ratio,
        ((point.1 as f32 / window_dimensions.1 as f32) - 0.5) * 2.0
    )
}

/// Objects able to be drawn by the renderer
pub enum DrawCommand<'a> {
    /// Text as represented by a layed-out Section
    Text(&'a glyph_brush::OwnedSection),
    /// A simple circle
    Circle {
        center: (i32, i32),
        radius: u32,
        color: crate::Color,
    },
    /// A simple rectangle
    Rect {
        top_left: (i32, i32),
        width: u32,
        height: u32,
        color: crate::Color,
    },
    /// Rectangle with rounded corners. Roundness is a percentage.
    RoundedRect {
        top_left: (i32, i32),
        width: u32,
        height: u32,
        roundness_percent: f32,
        color: crate::Color,
    },
    /// Sampled image resource
    Image {
        alias: &'a str,
        top_left: (i32, i32),
        width: u32,
        height: u32,
    },
}

/// Bundles Renderer with required context for an easy-to-use construct.
///
/// This is done to avoid needing to pass a context type to all `Renderer` functions 
/// (and to all view element render-functions by extension).
pub struct ContextualRenderer<'frame> {
    pub renderer: &'frame mut Renderer,
    pub device: &'frame wgpu::Device,
    // TODO: See whether `queue.write_buffer` can be ordered properly
    pub queue: &'frame wgpu::Queue,
    pub target: &'frame wgpu::TextureView,
    pub encoder: &'frame mut wgpu::CommandEncoder,
    pub window_dimensions: (u32, u32),
}

impl<'frame> ContextualRenderer<'frame> {
    pub fn draw(&mut self, command: DrawCommand) {
        self.renderer.draw(
            command, 
            self.device,
            self.queue,
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
    ui_quad: quad::Quad,
    // quad_bind_group_layout: wgpu::BindGroupLayout,
    quad_render_pipeline: wgpu::RenderPipeline,


    pub texture_map: texture::TextureMap,
    texture_quad: texture::TextureQuad,
    // texture_bind_group_layout: wgpu::BindGroupLayout,
    texture_render_pipeline: wgpu::RenderPipeline,

    pub text_renderer: font::TextRenderer,
}

// Reference: https://github.com/hecrj/iced/blob/master/wgpu/src/

impl Renderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, fonts: font::IncludedFonts, image_resources: crate::widget::IncludedImages) -> Self {
        let quad_bind_group_layout = crate::render::quad::Quad::bind_group_layout(device);
        let quad_render_pipeline = crate::render::quad::Quad::create_render_pipeline(device, &quad_bind_group_layout);
        // Default -- will be adjusted by render calls
        let ui_quad = quad::Quad::new(device, &quad_bind_group_layout);

        let texture_bind_group_layout = texture::Texture::bind_group_layout(device);
        let texture_render_pipeline = texture::Texture::create_render_pipeline(device, &texture_bind_group_layout);
        let texture_map = texture::TextureMap::from_images(
            image_resources, 
            &texture_bind_group_layout, 
            device, 
            queue
        );
        let texture_quad = texture::TextureQuad::new(device);

        let text_renderer = font::TextRenderer::from_fonts(
            fonts, 
            device, 
            crate::TEXTURE_FORMAT
        );

        Self {
            ui_quad,
            // quad_bind_group_layout,
            quad_render_pipeline,

            texture_map,
            texture_quad,
            // texture_bind_group_layout,
            texture_render_pipeline,

            text_renderer,
        }
    }

    pub fn draw(&mut self, command: DrawCommand, device: &wgpu::Device, _queue: &wgpu::Queue, target: &wgpu::TextureView, encoder: &mut wgpu::CommandEncoder, window_dimensions: (u32, u32)) {       
        match command {
            DrawCommand::Circle { center, radius, color } => {
                self.ui_quad.update_vertices(device, window_dimensions, (center.0 - radius as i32, center.1 - radius as i32), radius*2, radius*2);
                self.ui_quad.update_uniforms(device, encoder, quad::Uniforms {
                    color: color.into(),
                    window_dimensions: (window_dimensions.0 as f32, window_dimensions.1 as f32).into(),
                    center: (center.0 as f32, center.1 as f32).into(),
                    primitive_type: primitive::CIRCLE,
                    circle_radius: radius as f32,
                    primitive_width: 0.0,
                    primitive_height: 0.0,
                    rounded_rect_roundness: 0.0,
                });
            }

            DrawCommand::Rect { top_left, width, height, color } => {
                self.ui_quad.update_vertices(device, window_dimensions, top_left, width, height);
                self.ui_quad.update_uniforms(device, encoder,quad::Uniforms {
                    window_dimensions: (window_dimensions.0 as f32, window_dimensions.1 as f32).into(),
                    color: color.into(),
                    primitive_type: primitive::RECTANGLE,
                    center: (0.0, 0.0).into(),
                    circle_radius: 0.0,
                    primitive_width: 0.0,
                    primitive_height: 0.0,
                    rounded_rect_roundness: 0.0,
                });
            }

            DrawCommand::RoundedRect { top_left, width, height, mut roundness_percent, color } => {
                self.ui_quad.update_vertices(device, window_dimensions, top_left, width, height);
                
                // roundness = clamp(min(half_width, half_height), 0, 100)
                if roundness_percent < 0.0 {roundness_percent = 0.0;} else if roundness_percent > 100.0 {roundness_percent = 100.0;}
                let roundness = (0.01 * roundness_percent) * std::cmp::min::<u32>(width, height) as f32 / 2.0;
                
                self.ui_quad.update_uniforms(device, encoder, quad::Uniforms {
                    window_dimensions: (window_dimensions.0 as f32, window_dimensions.1 as f32).into(),
                    color: color.into(),
                    primitive_type: primitive::ROUNDED_RECTANGLE,
                    center: (
                        top_left.0 as f32 + width as f32 / 2.0, 
                        top_left.1 as f32 + height as f32 / 2.0,
                    ).into(),
                    circle_radius: 0.0,
                    primitive_width: width as f32 / 2.0,
                    primitive_height: height as f32 / 2.0,
                    rounded_rect_roundness: roundness,
                });
            }

            // Rendering is handled by the TextRenderer -> early return
            DrawCommand::Text(section) => {
                self.text_renderer.queue_section(section);
                return;
            }

            DrawCommand::Image { alias, top_left, width, height } => {
                self.texture_quad.update_vertices(device, window_dimensions, top_left, width, height);
                
                let mut render_pass = Self::create_render_pass(encoder, target);
                render_pass.set_pipeline(&self.texture_render_pipeline);
                
                let texture = self.texture_map.get(alias);
                self.texture_quad.render(&mut render_pass, texture);
                return;
            }
        } // match

        let mut render_pass = Self::create_render_pass(encoder, target);
        render_pass.set_pipeline(&self.quad_render_pipeline);
        self.ui_quad.render(&mut render_pass);
    }

    fn create_render_pass<'a>(encoder: &'a mut wgpu::CommandEncoder, target: &'a wgpu::TextureView) -> wgpu::RenderPass<'a> {
        encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            color_attachments: &[
                wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: true,
                    },
                },
            ],
            depth_stencil_attachment: None,
        })
    }
}