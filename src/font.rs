use wgpu_glyph::{ab_glyph, GlyphBrushBuilder, Section, FontId};

use crate::application::gpu;

use std::collections::HashMap;

// For reference, see:
// https://github.com/hecrj/coffee/blob/master/src/graphics/backend_wgpu/font.rs

// TODO: Allow for building text manually (position, size, etc.)
//       That id can then be referenced via Text::with_font_id(font_id)
// TODO: Need sizing functionality (text constraints) for element layout
pub struct TextRenderer {
    /// Contains all fonts. Used to render text
    brush: wgpu_glyph::GlyphBrush<()>,
    /// Map of (font_alias -> font_id)
    fonts: HashMap<&'static str, FontId>,
    // TODO: Consider adding a HashSet containing all the loaded font paths.
    // This would guarentee that no fonts are ever duplicated
}

impl TextRenderer {
    // From https://docs.rs/wgpu_glyph/0.9.0/wgpu_glyph/struct.GlyphBrushBuilder.html#method.using_font
    const DEFAULT_FONT_ID: FontId = FontId(0);

    /// Creates a `TextRenderer` with an aliased default font.
    /// 
    /// A default font is required, but this can be treated like adding any regular font via `add_font`
    pub fn from_default_font<P: AsRef<std::path::Path>>(default_font_alias: &'static str, font_path: P, device: &wgpu::Device, render_format: wgpu::TextureFormat) -> Self {
        let font_bytes = std::fs::read(font_path)
            .unwrap();
        let font = ab_glyph::FontArc::try_from_vec(font_bytes)
            .unwrap();

        let brush = GlyphBrushBuilder::using_font(font)
            .build(device, render_format);
        
        let mut fonts = HashMap::new();
        fonts.insert(default_font_alias, Self::DEFAULT_FONT_ID);

        Self {
            brush,
            fonts,
        }
    }

    // TODO: Ensure that this accounts for FontId properly
    /// For use with the `include_fonts!` macro
    pub fn from_fonts(fonts: Vec<(&'static str, ab_glyph::FontArc)>, device: &wgpu::Device, render_format: wgpu::TextureFormat) -> Self {
        let mut font_map = HashMap::new();

        let brush = GlyphBrushBuilder::using_fonts(
            fonts.into_iter().enumerate().map(|(index, (alias, font))| {
                font_map.insert(alias, FontId(index));
                font
            }).collect()
        ).build(device, render_format);

        Self {
            brush,
            fonts: font_map,
        }
    }

    /// Add a font for rendering. Font will be referred to by its unique, given alias.
    pub fn add_font<P: AsRef<std::path::Path>>(&mut self, alias: &'static str, font_path: P) {
        let font_bytes = std::fs::read(font_path)
            .unwrap();
        let font = ab_glyph::FontArc::try_from_vec(font_bytes)
            .unwrap();

        let id = self.brush.add_font(font);

        if let Some(FontId(existing_id)) = self.fonts.insert(alias, id) {
            panic!("A font with alias `{}` already exists (FontId: {})", alias, existing_id);
        }
    }

    /// Get the font_id for a registered font
    pub fn get_font_id(&self, alias: &str) -> FontId {
        if let Some(font_id) = self.fonts.get(alias) {
            *font_id
        } else {
            panic!("The font alias `{}` is not registered", alias);
        }
    }

    // TODO: Auto-generate Section from formatted text (like markdown)
    // TEMP: This will eventually be replaced with a simple builder allowing for easy placement/configuration
    pub fn render_section(&mut self, wgpu: &mut gpu, target_texture_view: &wgpu::TextureView, command_buffers: &mut Vec<wgpu::CommandBuffer>, target_width: u32, target_height: u32, section: Section) {
        // TODO: Find a convenient way to size text using the following
        use wgpu_glyph::GlyphCruncher;
        let bounds = self.brush.glyph_bounds(&section).unwrap();
        println!("{:?}", bounds);
        
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

        command_buffers.push(encoder.finish());

        // wgpu.queue.submit(&[encoder.finish()]);
    }
}

/// Load fonts as bytes from paths (embeds fonts in program)
///
/// Generates a list of (alias, font) for use with TextRenderer
///
/// Usage:
/// ```
/// let fonts = include_fonts! {
///     alias_1 => "path/to/font1",
///     alias_2 => "path/to/font2", ...
/// };
/// ```
#[macro_export]
macro_rules! include_fonts {
    ( $($alias:ident => $font_path:expr),+ $(,)? ) => {{
        let mut fonts = Vec::new();

        $(
            let font_bytes = include_bytes!($font_path);
            let font = wgpu_glyph::ab_glyph::FontArc::try_from_slice(font_bytes).unwrap();

            fonts.push((stringify!($alias), font));
        )+

        fonts
    };
}}