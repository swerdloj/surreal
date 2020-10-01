use wgpu::*;
use wgpu::util::{DeviceExt, BufferInitDescriptor, make_spirv};
// use wgpu::Texture as wgpu_Texture;

pub struct TextureMap {
    textures: std::collections::HashMap<&'static str, Texture>,
}

impl TextureMap {
    pub fn from_images(images: crate::widget::IncludedImages, bind_group_layout: &BindGroupLayout, device: &Device, queue: &Queue) -> Self {
        let mut textures = std::collections::HashMap::new();

        for (alias, image) in images {
            let texture = Texture::new(image, bind_group_layout, device, queue);
            textures.insert(alias, texture);
        }

        Self {
            textures,
        }
    }

    pub fn get<'a>(&'a self, alias: &str) -> &'a Texture {
        if let Some(texture) = self.textures.get(alias) {
            texture
        } else {
            panic!("No such texture exists: `{}`", alias);
        }
    }

    pub fn get_resource_dimensions(&self, alias: &str) -> (u32, u32) {
        let texture = self.get(alias);

        (texture.width, texture.height)
    }
}

// TODO: Uniforms
pub(super) struct TextureQuad {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl TextureQuad {
    // TODO: Account for texture coords
    fn vertices_from_rect(window_dimensions: (u32, u32), mut top_left: (i32, i32), width: u32, height: u32) -> [TextureVertex; 4] {
        // Make top-left (0, 0) where down is +y-axis
        top_left.1 = window_dimensions.1 as i32 - top_left.1;
        
        let quad_top_left = super::screen_space_to_draw_space(top_left, window_dimensions);
        
        let top_right = (top_left.0 + width as i32, top_left.1);
        let quad_top_right = super::screen_space_to_draw_space(top_right, window_dimensions);
        
        let bottom_left = (top_left.0, top_left.1 - height as i32);
        let quad_bottom_left = super::screen_space_to_draw_space(bottom_left, window_dimensions);
        
        let bottom_right = (top_left.0 + width as i32, top_left.1 - height as i32);
        let quad_bottom_right = super::screen_space_to_draw_space(bottom_right, window_dimensions);
    
        [
            TextureVertex::new((quad_top_right.0, quad_top_right.1, 0.0), (1.0, 1.0)), // Top right
            TextureVertex::new((quad_top_left.0, quad_top_left.1, 0.0), (0.0, 1.0)), // Top left
            TextureVertex::new((quad_bottom_left.0, quad_bottom_left.1, 0.0), (0.0, 0.0)), // Bottom left
            TextureVertex::new((quad_bottom_right.0, quad_bottom_right.1, 0.0), (1.0, 0.0)), // Bottom right
        ]
    }

    pub fn new(device: &Device) -> Self {
        let index_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("texture_quad_index_buffer"),
            contents: bytemuck::cast_slice(&[0u32, 1, 2, 0, 2, 3]), 
            usage: BufferUsage::INDEX,
        });

        let vertices = Self::vertices_from_rect((1,1), (1,1), 1, 1);

        let vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("quad_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsage::VERTEX, // | BufferUsage::COPY_DST,
        });

        Self {
            vertex_buffer,
            index_buffer,
        }
    }

    pub fn update_vertices(&mut self, device: &Device, window_dimensions: (u32, u32), top_left: (i32, i32), width: u32, height: u32) {
        let vertices = Self::vertices_from_rect(window_dimensions, top_left, width, height);
        
        // FIXME: Should I use a staging buffer or map_write() instead?
        // FIXME: Queue::write_buffer is ideal, but doesn't work
        self.vertex_buffer = device.create_buffer_init(&BufferInitDescriptor {
            label: Some("quad_vertex_buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: BufferUsage::VERTEX, // BufferUsage::COPY_SRC
        });
    }

    // Data always outlives render_pass
    pub fn render<'a, 'b>(&'a mut self, render_pass: &mut RenderPass<'b>, texture: &'a Texture) where 'a: 'b {
        render_pass.set_bind_group(0, &texture.bind_group, &[]);
        render_pass.set_index_buffer(self.index_buffer.slice(..));
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw_indexed(0..6, 0, 0..1);
    }
}


#[repr(C)]
#[derive(Copy, Clone)]
/// repr(C)
struct TextureVertex {
    position: cgmath::Vector3<f32>,
    texture_coord: cgmath::Vector2<f32>,
}

impl TextureVertex {
    pub fn new(position: (f32, f32, f32), texture_coord: (f32, f32)) -> Self {
        Self {
            position: position.into(),
            texture_coord: texture_coord.into(),
        }
    }

    pub fn descriptor<'a>() -> VertexBufferDescriptor<'a> {
        // FIXME: Can't use the macro  (caused by VertexFormat::TYPE.size())
        // let attributes = vertex_attr_array![
        //     0 => Float3,
        //     1 => Float2
        // ];

        VertexBufferDescriptor {
            stride: size_of!(Self),
            step_mode: InputStepMode::Vertex,
            attributes: &[
                VertexAttributeDescriptor {
                    offset: 0,
                    format: VertexFormat::Float3,
                    shader_location: 0,
                },
                VertexAttributeDescriptor {
                    offset: size_of!([f32; 3]),
                    format: VertexFormat::Float2,
                    shader_location: 1,
                },
            ],
        }
    }
}

unsafe impl bytemuck::Pod for TextureVertex {}
unsafe impl bytemuck::Zeroable for TextureVertex {}


// References: https://github.com/sotrh/learn-wgpu/blob/master/code/intermediate/tutorial13-threading/src/model.rs
// &           https://github.com/sotrh/learn-wgpu/blob/master/code/intermediate/tutorial13-threading/src/texture.rs
// &           https://sotrh.github.io/learn-wgpu/beginner/tutorial5-textures/#cleaning-things-up

pub struct Texture {
    width: u32,
    height: u32,
    // texture: wgpu_Texture,
    // view: TextureView,
    // sampler: Sampler,
    bind_group: BindGroup,
}

impl Texture {
    pub fn new(image_resource: image::DynamicImage, bind_group_layout: &BindGroupLayout, device: &Device, queue: &Queue) -> Self {
        let rgba_image = image_resource.as_rgba8().unwrap();

        let (width, height) = rgba_image.dimensions();

        let size = Extent3d {
            width,
            height,
            depth: 1, // 2d texture
        };

        let texture = device.create_texture(&TextureDescriptor {
            label: Some("texture_resource"),
            size,
            mip_level_count: 1,
            sample_count: 1,
            dimension: TextureDimension::D2,
            format: crate::TEXTURE_FORMAT,
            // Sampled => use in shaders
            usage: TextureUsage::SAMPLED | TextureUsage::COPY_DST,
        });

        // Writes to texture from buffer (rgba_image)
        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            rgba_image,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * width, // RGBA8
                rows_per_image: height,
            },
            size
        );

        // FIXME: Either use the same sampler for all images, or give user some control
        //        over this. Making the same sampler for each image is wasteful.
        let sampler = device.create_sampler(&SamplerDescriptor {
            label: Some("texture_sampler"),
            address_mode_u: AddressMode::ClampToEdge,
            address_mode_v: AddressMode::ClampToEdge,
            address_mode_w: AddressMode::ClampToEdge,
            mag_filter: FilterMode::Linear,
            min_filter: FilterMode::Linear,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let bind_group = Self::bind_group(device, bind_group_layout, &view, &sampler);

        Self {
            width,
            height,
            // view,
            // texture,
            // sampler,
            bind_group,
        }
    }

    // This must be saved and used whenever needed (only one instance may exist of this)
    pub(crate) fn bind_group_layout(device: &Device) -> BindGroupLayout {
        device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("texture_bind_group_layout"),
            entries: &[
                // Texture
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::SampledTexture {
                        dimension: TextureViewDimension::D2,
                        component_type: TextureComponentType::Uint,
                        multisampled: false,
                    },
                    count: None,
                },
                // Sampler
                BindGroupLayoutEntry {
                    binding: 1,
                    visibility: ShaderStage::FRAGMENT,
                    ty: BindingType::Sampler {
                        comparison: false,
                    },
                    count: None,
                }
            ],
        })
    }

    pub(crate) fn bind_group(device: &Device, layout: &BindGroupLayout, texture_view: &TextureView, sampler: &Sampler) -> BindGroup {
        device.create_bind_group(&BindGroupDescriptor {
            label: Some("texture_bind_group"),
            layout: &layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: BindingResource::TextureView(texture_view),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: BindingResource::Sampler(&sampler),
                    
                }
            ],
        })
    }

    pub(crate) fn create_render_pipeline(device: &Device, bind_group_layout: &BindGroupLayout) -> RenderPipeline {
        use std::io::Read;
        let mut spirv_buffer1 = Vec::new();
        let mut spirv_buffer2 = Vec::new();

        let vert_shader = include_str!("../../shaders/image/texture.vert");
        let mut vert_spirv = glsl_to_spirv::compile(vert_shader, glsl_to_spirv::ShaderType::Vertex).unwrap();
        vert_spirv.read_to_end(&mut spirv_buffer1).unwrap();
        let vert_data = make_spirv(&spirv_buffer1);
        
        let frag_shader = include_str!("../../shaders/image/texture.frag");
        let mut frag_spirv = glsl_to_spirv::compile(frag_shader, glsl_to_spirv::ShaderType::Fragment).unwrap();
        frag_spirv.read_to_end(&mut spirv_buffer2).unwrap();
        let frag_data = make_spirv(&spirv_buffer2);

        let vert_module = device.create_shader_module(vert_data);
        let frag_module = device.create_shader_module(frag_data);
        
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("texture_pipeline_layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });
        
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("texture_render_pipeline"),
            layout: Some(&pipeline_layout),
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
                cull_mode: CullMode::None,
                ..Default::default()
            }),
            primitive_topology: PrimitiveTopology::TriangleList,
            // Simple alpha blending
            color_states: &[ColorStateDescriptor {
                format: crate::TEXTURE_FORMAT,
                color_blend: BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,                 
                },
                alpha_blend: BlendDescriptor {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Add,                 
                },
                write_mask: ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: VertexStateDescriptor {
                index_format: IndexFormat::Uint32,
                vertex_buffers: &[
                    TextureVertex::descriptor(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        })
    }
}