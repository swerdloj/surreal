use wgpu::*;
// use wgpu::util::{DeviceExt, BufferInitDescriptor};
use wgpu::Texture as wgpu_Texture;

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
}

pub(super) struct TextureQuad {
    vertex_buffer: Buffer,
    index_buffer: Buffer,
}

impl TextureQuad {
    pub fn new() -> Self {
        todo!()
    }

    pub fn update_vertices(&mut self, device: &Device, window_dimensions: (u32, u32), top_left: (i32, i32), width: u32, height: u32) {
        todo!()
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
    texture: wgpu_Texture,
    view: TextureView,
    sampler: Sampler,
    bind_group: BindGroup,
}

// TODO: Pipelines
// TODO: Vertices
// TODO: Vertex Buffers
// TODO: Uniforms

// TODO: Use a similar approach to quads, maintaining only one instance and
// swapping around the vertex buffer and contents

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
            min_filter: FilterMode::Nearest,
            mipmap_filter: FilterMode::Nearest,
            ..Default::default()
        });

        let view = texture.create_view(&TextureViewDescriptor::default());

        let bind_group = Self::bind_group(device, bind_group_layout, &view, &sampler);

        Self {
            width,
            height,
            view,
            texture,
            sampler,
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
        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("texture_pipeline_layout"),
            bind_group_layouts: &[bind_group_layout],
            push_constant_ranges: &[],
        });
        
        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("texture_render_pipeline"),
            layout: Some(&pipeline_layout),
            vertex_stage: todo!(),
            fragment_stage: todo!(),
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