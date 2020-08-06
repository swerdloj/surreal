use wgpu::*;

use crate::texture;

/// Note that quads are not translated or rotated by default
pub struct Quad {
    /// Quad's vertices
    pub vertex_buffer: Buffer,
    /// Quad's vertex indices
    pub index_buffer: Buffer,
    /// Contains wgpu texture, view, and sampler
    texture: texture::Texture,

    /// Uniform bind group ready to be passed
    pub bind_group: BindGroup,
}

impl Quad {
    // TODO: Create quad with specified pixel dimensions/location
    // TODO: Need to accont for window dimensions/aspect ratio

    /// Creates quad sized according to the texture's dimensions
    pub fn new(device: &Device, layout: &BindGroupLayout, texture: texture::Texture, size: Option<(u32, u32)>) -> Self {
        let mut x = 1.0;
        let mut y = 1.0;

        
        if let Some((width, height)) = texture.image_dimensions {
            let aspect_ratio = width as f32 / height as f32;
            if width > height {
                y = 1.0 / aspect_ratio;
            } else {
                x = 1.0 * aspect_ratio;
            }
        }
        

        let vertices = [
            QuadVertex::new(( x,  y, 0.0), (1.0, 1.0)), // Top right
            QuadVertex::new((-x,  y, 0.0), (0.0, 1.0)), // Top left
            QuadVertex::new((-x, -y, 0.0), (0.0, 0.0)), // Bottom left
            QuadVertex::new(( x, -y, 0.0), (1.0, 0.0)), // Bottom right
        ];

        Self::create(device, layout, texture, vertices)
    }

    pub fn new_full_screen(device: &Device, layout: &BindGroupLayout, texture: texture::Texture) -> Self {
        let vertices = [
            QuadVertex::new(( 1.0,  1.0, 0.0), (1.0, 1.0)), // Top right
            QuadVertex::new((-1.0,  1.0, 0.0), (0.0, 1.0)), // Top left
            QuadVertex::new((-1.0, -1.0, 0.0), (0.0, 0.0)), // Bottom left
            QuadVertex::new(( 1.0, -1.0, 0.0), (1.0, 0.0)), // Bottom right
        ];

        Self::create(device, layout, texture, vertices)
    }

    fn create(device: &Device, layout: &BindGroupLayout, texture: texture::Texture, vertices: [QuadVertex; 4]) -> Self {
        // 2x ccw triangle vertex indices
        let indices = [0u32, 1, 2, 0, 2, 3]; // (topR -> topL, botL), (topR, botL, botR)

        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[vertices]),
            BufferUsage::VERTEX
        );

        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[indices]), 
            BufferUsage::INDEX,
        );

        let bind_group = Self::bind_group(device, layout, &texture);

        Self {
            vertex_buffer,
            index_buffer,
            texture,
            bind_group,
        }
    }

    /// NOTE: This must be saved for ALL use cases after being created
    ///
    /// All quads must use the same instance of this layout
    pub fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            bindings: &[
                // Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::SampledTexture {
                        multisampled: false,
                        dimension: TextureViewDimension::D2,
                        component_type: TextureComponentType::Uint,
                    },
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        comparison: false,
                    },
                },
            ],
            label: Some("quad_texture_bind_group_layout"),
        })
    }

    /// Unlike layouts, the `bind_group` is unique to each object (but linked to the layout)
    pub fn bind_group(device: &Device, layout: &BindGroupLayout, texture: &texture::Texture) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            layout,
            bindings: &[
                // Texture
                Binding {
                    binding: 0,
                    resource: BindingResource::TextureView(&texture.view),
                },
                // Sampler
                Binding {
                    binding: 1,
                    resource: BindingResource::Sampler(&texture.sampler),
                }
            ],
            label: Some("quad_bind_group"),
        })
    }

    pub fn create_render_pipeline(device: &Device, 
                                  layout: &BindGroupLayout,
                                  color_format: TextureFormat,
    ) -> RenderPipeline {
        let layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            bind_group_layouts: &[
                layout,
            ],
        });

        let vert_spirv = include_bytes!("../shaders/quad/quad.vert.spv");
        let vert_data = read_spirv(std::io::Cursor::new(vert_spirv.as_ref())).unwrap();

        let frag_spirv = include_bytes!("../shaders/quad/quad.frag.spv");
        let frag_data = read_spirv(std::io::Cursor::new(frag_spirv.as_ref())).unwrap();

        let vert_module = device.create_shader_module(&vert_data);
        let frag_module = device.create_shader_module(&frag_data);

        device.create_render_pipeline(&RenderPipelineDescriptor {
            layout: &layout,
            vertex_stage: ProgrammableStageDescriptor {
                module: &vert_module,
                entry_point: "main",
            },
            fragment_stage: Some(ProgrammableStageDescriptor {
                module: &frag_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            color_states: &[
                ColorStateDescriptor {
                    format: color_format,
                    alpha_blend: BlendDescriptor::REPLACE,
                    color_blend: BlendDescriptor::REPLACE,
                    write_mask: ColorWrite::ALL,
                },
            ],
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint32,
                vertex_buffers: &[
                    QuadVertex::descriptor(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
/// repr(C)
struct QuadVertex {
    position: cgmath::Vector3<f32>,
    tex_coord: cgmath::Vector2<f32>,
}

unsafe impl bytemuck::Pod for QuadVertex {}
unsafe impl bytemuck::Zeroable for QuadVertex {}

impl QuadVertex {
    pub fn new(position: (f32, f32, f32), tex_coord: (f32, f32)) -> Self {
        Self {
            position: position.into(),
            tex_coord: tex_coord.into(),
        }
    }

    pub fn descriptor<'a>() -> VertexBufferDescriptor<'a> {
        VertexBufferDescriptor {
            stride: size_of!(Self) as _,
            step_mode: InputStepMode::Vertex,
            attributes: &vertex_attr_array![
                // Vertex location
                0 => Float3,
                // Texture coords
                1 => Float2
            ],
        }
    }
}