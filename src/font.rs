use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, Text};

use crate::application::gpu;

// For reference, see:
// https://github.com/hecrj/coffee/blob/master/src/graphics/backend_wgpu/font.rs

// TODO: Allow multiple fonts. Allow for building text manually (position, size, etc.)
pub struct TextRenderer {
    brush: wgpu_glyph::GlyphBrush<()>,
}

impl TextRenderer {
    pub fn new<P: AsRef<std::path::Path>>(font_path: P, device: &wgpu::Device, render_format: wgpu::TextureFormat) -> Self {
        let font_bytes = std::fs::read(font_path)
            .unwrap();
        let font = ab_glyph::FontArc::try_from_vec(font_bytes)
            .unwrap();

        let brush = GlyphBrushBuilder::using_font(font)
            .build(device, render_format);
        
        Self {
            brush,
        }
    }

    pub fn render_text(&mut self, wgpu: &mut gpu, target_texture_view: &wgpu::TextureView, target_width: u32, target_height: u32, text: &str) {
        let section = Section {
            screen_position: (10.0, 10.0),
            text: vec![
                Text::new(text)
                    .with_scale(25.0)
                    .with_color([1.0, 1.0, 1.0, 1.0])
            ],
            ..Section::default()
        };

        self.brush.queue(section);

        let mut encoder = wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("text_renderer_encoder"),
        });

        self.brush.draw_queued(
            &wgpu.device,
            &mut encoder,
            target_texture_view,
            target_width,
            target_height,
        ).unwrap();

        wgpu.queue.submit(&[encoder.finish()]);
    }
}